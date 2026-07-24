import { useEffect, useState } from 'react';
import { api } from '../lib/api';
import { useAuthGuard } from '../lib/useAuthGuard';

const emptyForm = { nombre: '', direccion: '', cuit: '', telefono: '' };

export default function EmpresaApp() {
	const ready = useAuthGuard();
	const [form, setForm] = useState(emptyForm);
	const [loading, setLoading] = useState(true);
	const [saving, setSaving] = useState(false);
	const [error, setError] = useState<string | null>(null);
	const [saved, setSaved] = useState(false);

	useEffect(() => {
		if (!ready) return;
		api.empresa
			.get()
			.then((empresa) =>
				setForm({
					nombre: empresa.nombre,
					direccion: empresa.direccion,
					cuit: empresa.cuit,
					telefono: empresa.telefono,
				})
			)
			.catch(() => setError('No se pudieron cargar los datos de la empresa'))
			.finally(() => setLoading(false));
	}, [ready]);

	async function handleSubmit(e: React.FormEvent) {
		e.preventDefault();
		setError(null);
		setSaved(false);
		setSaving(true);
		try {
			await api.empresa.update(form);
			setSaved(true);
		} catch {
			setError('No se pudieron guardar los datos de la empresa');
		} finally {
			setSaving(false);
		}
	}

	if (!ready) return null;

	return (
		<div className="max-w-4xl mx-auto p-4 flex flex-col gap-6">
			<h1 className="text-2xl font-bold text-slate-800">Mi empresa</h1>
			<p className="text-sm text-slate-500 -mt-4">
				Estos datos aparecen en la columna izquierda de cada remito generado.
			</p>

			{loading ? (
				<p className="text-slate-500">Cargando...</p>
			) : (
				<form
					onSubmit={handleSubmit}
					className="bg-white rounded-lg shadow p-4 flex flex-col gap-3"
				>
					<div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
						<input
							placeholder="Nombre / Razon social"
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
							placeholder="CUIT"
							value={form.cuit}
							onChange={(e) => setForm({ ...form, cuit: e.target.value })}
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
					{saved && <p className="text-sm text-green-600">Datos guardados.</p>}

					<button
						type="submit"
						disabled={saving}
						className="bg-slate-800 text-white rounded px-4 py-2 font-medium hover:bg-slate-700 disabled:opacity-50 self-start"
					>
						{saving ? 'Guardando...' : 'Guardar'}
					</button>
				</form>
			)}
		</div>
	);
}
