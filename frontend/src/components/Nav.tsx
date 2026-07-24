import { useEffect, useState } from 'react';
import { api } from '../lib/api';

const links = [
	{ href: '/', label: 'Inicio' },
	{ href: '/clientes', label: 'Clientes' },
	{ href: '/productos', label: 'Productos' },
	{ href: '/remitos', label: 'Remitos' },
	{ href: '/empresa', label: 'Mi empresa' },
];

export default function Nav() {
	const [pathname, setPathname] = useState('');

	useEffect(() => {
		setPathname(window.location.pathname);
	}, []);

	async function handleLogout() {
		await api.auth.logout();
		window.location.href = '/login';
	}

	return (
		<nav className="bg-slate-800 text-white">
			<div className="max-w-4xl mx-auto flex items-center justify-between px-4 py-3">
				<div className="flex gap-4">
					{links.map((link) => (
						<a
							key={link.href}
							href={link.href}
							className={`text-sm font-medium ${
								pathname === link.href
									? 'text-white'
									: 'text-slate-300 hover:text-white'
							}`}
						>
							{link.label}
						</a>
					))}
				</div>
				<button
					onClick={handleLogout}
					className="text-sm text-slate-300 hover:text-white"
				>
					Cerrar sesion
				</button>
			</div>
		</nav>
	);
}
