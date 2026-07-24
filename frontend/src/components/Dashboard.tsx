import { useAuthGuard } from '../lib/useAuthGuard';

const cards = [
	{
		href: '/clientes',
		title: 'Clientes',
		description: 'Cargar y editar los datos de tus clientes.',
	},
	{
		href: '/productos',
		title: 'Productos',
		description: 'Cargar tus productos y precios.',
	},
	{
		href: '/remitos',
		title: 'Remitos',
		description: 'Generar remitos en PDF para enviar o imprimir.',
	},
	{
		href: '/empresa',
		title: 'Mi empresa',
		description: 'Cargar los datos de tu empresa que aparecen en los remitos.',
	},
];

export default function Dashboard() {
	const ready = useAuthGuard();

	if (!ready) return null;

	return (
		<main className="max-w-4xl mx-auto p-4">
			<h1 className="text-2xl font-bold text-slate-800 mb-6">
				Sistema de Remitos
			</h1>
			<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
				{cards.map((card) => (
					<a
						key={card.href}
						href={card.href}
						className="bg-white rounded-lg shadow p-5 hover:shadow-md transition-shadow"
					>
						<h2 className="font-semibold text-slate-800 mb-1">{card.title}</h2>
						<p className="text-sm text-slate-500">{card.description}</p>
					</a>
				))}
			</div>
		</main>
	);
}
