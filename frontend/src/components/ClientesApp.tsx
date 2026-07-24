import { useEffect, useState } from 'react';
import { api } from '../lib/api';
import { useAuthGuard } from '../lib/useAuthGuard';
import type { Cliente } from '../lib/types';

const emptyForm = { nombre: '', direccion: '', cuit_dni: '', telefono: '' };

export default function ClientesApp() {
	const ready = useAuthGuard();
	const [clientes, setClientes] = useState<Cliente[]>([]);
	const [loading, setLoading] = useState(true);
	const [editingId, setEditingId] = useState<number | null>(null);
	const [form, setForm] = useState(emptyForm);
	const [error, setError] = useState<string | null>(null);
	const [saving, setSaving] = useState(false);

	async function loadClientes() {
		setLoading(true);
		try {
			setClientes(await api.clientes.list());
		} catch {
			setError('No se pudieron cargar los clientes');
		} finally {
			setLoading(false);
		}
	}

	useEffect(() => {
		if (ready) loadClientes();
	}, [ready]);

	function startEdit(cliente: Cliente) {
		setEditingId(cliente.id);
		setForm({
			nombre: cliente.nombre,
			direccion: cliente.direccion,
			cuit_dni: cliente.cuit_dni,
			telefono: cliente.telefono,
		});
	}

	function cancelEdit() {
		setEditingId(null);
		setForm(emptyForm);
	}

	async function handleSubmit(e: React.FormEvent) {
		e.preventDefault();
		setError(null);
		setSaving(true);

		try {
			if (editingId) {
				await api.clientes.update(editingId, form);
			} else {
				await api.clientes.create(form);
			}
			cancelEdit();
			await loadClientes();
		} catch {
			setError('No se pudo guardar el cliente');
		} finally {
			setSaving(false);
		}
	}

	async function handleDelete(id: number) {
		if (!confirm('Eliminar este cliente?')) return;
		try {
			await api.clientes.remove(id);
			await loadClientes();
		} catch {
			setError('No se pudo eliminar el cliente');
		}
	}

	if (!ready) return null;

	return (
		<div className="max-w-4xl mx-auto p-4 flex flex-col gap-6">
			<h1 className="text-2xl font-bold text-slate-800">Clientes</h1>

			<form
				onSubmit={handleSubmit}
				className="bg-white rounded-lg shadow p-4 flex flex-col gap-3"
			>
				<h2 className="font-semibold text-slate-700">
					{editingId ? 'Editar cliente' : 'Nuevo cliente'}
				</h2>

				<div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
					<input
						placeholder="Nombre"
						value={form.nombre}
						onChange={(e) => setForm({ ...form, nombre: e.target.value })}
						className="border border-slate-300 rounded px-3 py-2"
						required
					/>
					<input
						placeholder="Direccion"
						value={form.direccion}
						onChange={(e) => setForm({ ...form, direccion: e.target.value })}
						className="border border-slate-300 rounded px-3 py-2"
						required
					/>
					<input
						placeholder="CUIT / DNI"
						value={form.cuit_dni}
						onChange={(e) => setForm({ ...form, cuit_dni: e.target.value })}
						className="border border-slate-300 rounded px-3 py-2"
						required
					/>
					<input
						placeholder="Telefono"
						value={form.telefono}
						onChange={(e) => setForm({ ...form, telefono: e.target.value })}
						className="border border-slate-300 rounded px-3 py-2"
						required
					/>
				</div>

				{error && <p className="text-sm text-red-600">{error}</p>}

				<div className="flex gap-2">
					<button
						type="submit"
						disabled={saving}
						className="bg-slate-800 text-white rounded px-4 py-2 font-medium hover:bg-slate-700 disabled:opacity-50"
					>
						{saving ? 'Guardando...' : editingId ? 'Guardar cambios' : 'Crear cliente'}
					</button>
					{editingId && (
						<button
							type="button"
							onClick={cancelEdit}
							className="text-slate-600 px-4 py-2"
						>
							Cancelar
						</button>
					)}
				</div>
			</form>

			<div className="bg-white rounded-lg shadow overflow-x-auto">
				{loading ? (
					<p className="p-4 text-slate-500">Cargando...</p>
				) : clientes.length === 0 ? (
					<p className="p-4 text-slate-500">Todavia no hay clientes cargados.</p>
				) : (
					<table className="w-full text-sm">
						<thead>
							<tr className="bg-slate-100 text-left text-slate-600">
								<th className="p-3">Nombre</th>
								<th className="p-3">Direccion</th>
								<th className="p-3">CUIT/DNI</th>
								<th className="p-3">Telefono</th>
								<th className="p-3"></th>
							</tr>
						</thead>
						<tbody>
							{clientes.map((cliente) => (
								<tr key={cliente.id} className="border-t border-slate-100">
									<td className="p-3 font-medium text-slate-800">
										{cliente.nombre}
									</td>
									<td className="p-3 text-slate-600">{cliente.direccion}</td>
									<td className="p-3 text-slate-600">{cliente.cuit_dni}</td>
									<td className="p-3 text-slate-600">{cliente.telefono}</td>
									<td className="p-3 text-right whitespace-nowrap">
										<button
											onClick={() => startEdit(cliente)}
											className="text-slate-600 hover:text-slate-900 mr-3"
										>
											Editar
										</button>
										<button
											onClick={() => handleDelete(cliente.id)}
											className="text-red-600 hover:text-red-800"
										>
											Eliminar
										</button>
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
