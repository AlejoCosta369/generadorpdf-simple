import { useState } from 'react';
import { api } from '../lib/api';

export default function LoginForm() {
	const [username, setUsername] = useState('');
	const [password, setPassword] = useState('');
	const [error, setError] = useState<string | null>(null);
	const [loading, setLoading] = useState(false);

	async function handleSubmit(e: React.FormEvent) {
		e.preventDefault();
		setError(null);
		setLoading(true);
		try {
			await api.auth.login(username, password);
			window.location.href = '/';
		} catch {
			setError('Usuario o contrasena incorrectos');
		} finally {
			setLoading(false);
		}
	}

	return (
		<form
			onSubmit={handleSubmit}
			className="w-full max-w-sm bg-white rounded-lg shadow p-6 flex flex-col gap-4"
		>
			<h1 className="text-xl font-bold text-slate-800">Iniciar sesion</h1>

			<div className="flex flex-col gap-1">
				<label className="text-sm font-medium text-slate-600">Usuario</label>
				<input
					type="text"
					value={username}
					onChange={(e) => setUsername(e.target.value)}
					className="border border-slate-300 rounded px-3 py-2"
					required
				/>
			</div>

			<div className="flex flex-col gap-1">
				<label className="text-sm font-medium text-slate-600">
					Contrasena
				</label>
				<input
					type="password"
					value={password}
					onChange={(e) => setPassword(e.target.value)}
					className="border border-slate-300 rounded px-3 py-2"
					required
				/>
			</div>

			{error && <p className="text-sm text-red-600">{error}</p>}

			<button
				type="submit"
				disabled={loading}
				className="bg-slate-800 text-white rounded px-3 py-2 font-medium hover:bg-slate-700 disabled:opacity-50"
			>
				{loading ? 'Ingresando...' : 'Ingresar'}
			</button>
		</form>
	);
}
