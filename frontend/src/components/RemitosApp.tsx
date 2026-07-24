import { useEffect, useMemo, useState } from 'react';
import { api, formatPesos } from '../lib/api';
import { useAuthGuard } from '../lib/useAuthGuard';
import type { Cliente, Producto, Remito } from '../lib/types';

interface Line {
	productoId: number | '';
	cantidad: string;
}

export default function RemitosApp() {
	const ready = useAuthGuard();
	const [clientes, setClientes] = useState<Cliente[]>([]);
	const [productos, setProductos] = useState<Producto[]>([]);
	const [remitos, setRemitos] = useState<Remito[]>([]);
	const [loading, setLoading] = useState(true);
	const [clienteId, setClienteId] = useState<number | ''>('');
	const [lines, setLines] = useState<Line[]>([{ productoId: '', cantidad: '1' }]);
	const [error, setError] = useState<string | null>(null);
	const [saving, setSaving] = useState(false);

	async function loadAll() {
		setLoading(true);
		try {
			const [clientesData, productosData, remitosData] = await Promise.all([
				api.clientes.list(),
				api.productos.list(),
				api.remitos.list(),
			]);
			setClientes(clientesData);
			setProductos(productosData);
			setRemitos(remitosData);
		} catch {
			setError('No se pudieron cargar los datos');
		} finally {
			setLoading(false);
		}
	}

	useEffect(() => {
		if (ready) loadAll();
	}, [ready]);

	function productoById(id: number | '') {
		return productos.find((p) => p.id === id);
	}

	const total = useMemo(() => {
		return lines.reduce((sum, line) => {
			const producto = productoById(line.productoId);
			const cantidad = parseInt(line.cantidad, 10);
			if (!producto || !cantidad || cantidad <= 0) return sum;
			return sum + producto.precio_centavos * cantidad;
		}, 0);
	}, [lines, productos]);

	function updateLine(index: number, patch: Partial<Line>) {
		setLines((prev) =>
			prev.map((line, i) => (i === index ? { ...line, ...patch } : line))
		);
	}

	function addLine() {
		setLines((prev) => [...prev, { productoId: '', cantidad: '1' }]);
	}

	function removeLine(index: number) {
		setLines((prev) => prev.filter((_, i) => i !== index));
	}

	async function handleSubmit(e: React.FormEvent) {
		e.preventDefault();
		setError(null);

		if (!clienteId) {
			setError('Elegi un cliente');
			return;
		}

		const items = lines
			.filter((line) => line.productoId !== '' && parseInt(line.cantidad, 10) > 0)
			.map((line) => ({
				producto_id: line.productoId as number,
				cantidad: parseInt(line.cantidad, 10),
			}));

		if (items.length === 0) {
			setError('Agrega al menos un producto con cantidad valida');
			return;
		}

		setSaving(true);
		try {
			const result = await api.remitos.create({ cliente_id: clienteId, items });
			setClienteId('');
			setLines([{ productoId: '', cantidad: '1' }]);
			await loadAll();
			window.open(`/api/remitos/${result.remito.id}/pdf`, '_blank');
		} catch {
			setError('No se pudo generar el remito');
		} finally {
			setSaving(false);
		}
	}

	function clienteNombre(id: number) {
		return clientes.find((c) => c.id === id)?.nombre ?? `#${id}`;
	}

	if (!ready) return null;

	return (
		<div className="max-w-4xl mx-auto p-4 flex flex-col gap-6">
			<h1 className="text-2xl font-bold text-slate-800">Remitos</h1>

			<form
				onSubmit={handleSubmit}
				className="bg-white rounded-lg shadow p-4 flex flex-col gap-4"
			>
				<h2 className="font-semibold text-slate-700">Nuevo remito</h2>

				<div className="flex flex-col gap-1">
					<label className="text-sm font-medium text-slate-600">Cliente</label>
					<select
						value={clienteId}
						onChange={(e) =>
							setClienteId(e.target.value ? parseInt(e.target.value, 10) : '')
						}
						className="border border-slate-300 rounded px-3 py-2"
						required
					>
						<option value="">Elegi un cliente</option>
						{clientes.map((cliente) => (
							<option key={cliente.id} value={cliente.id}>
								{cliente.nombre}
							</option>
						))}
					</select>
				</div>

				<div className="flex flex-col gap-2">
					<label className="text-sm font-medium text-slate-600">Productos</label>
					{lines.map((line, index) => {
						const producto = productoById(line.productoId);
						return (
							<div key={index} className="flex gap-2 items-center">
								<select
									value={line.productoId}
									onChange={(e) =>
										updateLine(index, {
											productoId: e.target.value
												? parseInt(e.target.value, 10)
												: '',
										})
									}
									className="border border-slate-300 rounded px-3 py-2 flex-1"
								>
									<option value="">Elegi un producto</option>
									{productos.map((p) => (
										<option key={p.id} value={p.id}>
											{p.nombre} - {formatPesos(p.precio_centavos)}
										</option>
									))}
								</select>
								<input
									type="number"
									min="1"
									value={line.cantidad}
									onChange={(e) =>
										updateLine(index, { cantidad: e.target.value })
									}
									className="border border-slate-300 rounded px-3 py-2 w-24"
								/>
								<span className="text-sm text-slate-500 w-24 text-right">
									{producto && line.cantidad
										? formatPesos(
												producto.precio_centavos *
													(parseInt(line.cantidad, 10) || 0)
											)
										: ''}
								</span>
								<button
									type="button"
									onClick={() => removeLine(index)}
									className="text-red-600 hover:text-red-800 text-sm"
								>
									Quitar
								</button>
							</div>
						);
					})}
					<button
						type="button"
						onClick={addLine}
						className="self-start text-sm text-slate-600 hover:text-slate-900"
					>
						+ Agregar producto
					</button>
				</div>

				<div className="text-right font-semibold text-slate-800">
					Total: {formatPesos(total)}
				</div>

				{error && <p className="text-sm text-red-600">{error}</p>}

				<button
					type="submit"
					disabled={saving}
					className="bg-slate-800 text-white rounded px-4 py-2 font-medium hover:bg-slate-700 disabled:opacity-50 self-start"
				>
					{saving ? 'Generando...' : 'Generar remito y descargar PDF'}
				</button>
			</form>

			<div className="bg-white rounded-lg shadow overflow-x-auto">
				<h2 className="font-semibold text-slate-700 p-4 pb-0">
					Remitos generados
				</h2>
				{loading ? (
					<p className="p-4 text-slate-500">Cargando...</p>
				) : remitos.length === 0 ? (
					<p className="p-4 text-slate-500">Todavia no generaste remitos.</p>
				) : (
					<table className="w-full text-sm mt-2">
						<thead>
							<tr className="bg-slate-100 text-left text-slate-600">
								<th className="p-3">N°</th>
								<th className="p-3">Cliente</th>
								<th className="p-3">Fecha</th>
								<th className="p-3">Total</th>
								<th className="p-3"></th>
							</tr>
						</thead>
						<tbody>
							{remitos.map((remito) => (
								<tr key={remito.id} className="border-t border-slate-100">
									<td className="p-3 text-slate-800">
										{remito.id.toString().padStart(6, '0')}
									</td>
									<td className="p-3 text-slate-600">
										{clienteNombre(remito.cliente_id)}
									</td>
									<td className="p-3 text-slate-600">{remito.fecha}</td>
									<td className="p-3 text-slate-600">
										{formatPesos(remito.total_centavos)}
									</td>
									<td className="p-3 text-right">
										<a
											href={`/api/remitos/${remito.id}/pdf`}
											target="_blank"
											className="text-slate-600 hover:text-slate-900"
										>
											Descargar PDF
										</a>
									</td>
								</tr>
							))}
						</tbody>
					</table>
				)}
			</div>
		</div>
	);
}
