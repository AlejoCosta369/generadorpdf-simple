import type {
	Cliente,
	ClienteInput,
	Empresa,
	EmpresaInput,
	Producto,
	ProductoInput,
	Remito,
	RemitoCompleto,
	RemitoInput,
} from './types';

const API_BASE = '/api';

async function request<T>(path: string, options: RequestInit = {}): Promise<T> {
	const res = await fetch(`${API_BASE}${path}`, {
		...options,
		credentials: 'include',
		headers: { 'Content-Type': 'application/json', ...options.headers },
	});

	if (!res.ok) {
		const text = await res.text().catch(() => '');
		throw new Error(text || `Error ${res.status}`);
	}

	if (res.status === 204) {
		return undefined as T;
	}

	return res.json() as Promise<T>;
}

export const api = {
	auth: {
		login: (username: string, password: string) =>
			request<{ id: number; username: string }>('/auth/login', {
				method: 'POST',
				body: JSON.stringify({ username, password }),
			}),
		logout: () => request<void>('/auth/logout', { method: 'POST' }),
		me: () => request<{ id: number; username: string }>('/auth/me'),
	},
	clientes: {
		list: () => request<Cliente[]>('/clientes'),
		create: (input: ClienteInput) =>
			request<Cliente>('/clientes', {
				method: 'POST',
				body: JSON.stringify(input),
			}),
		update: (id: number, input: ClienteInput) =>
			request<Cliente>(`/clientes/${id}`, {
				method: 'PUT',
				body: JSON.stringify(input),
			}),
		remove: (id: number) =>
			request<void>(`/clientes/${id}`, { method: 'DELETE' }),
	},
	empresa: {
		get: () => request<Empresa>('/empresa'),
		update: (input: EmpresaInput) =>
			request<Empresa>('/empresa', {
				method: 'PUT',
				body: JSON.stringify(input),
			}),
	},
	productos: {
		list: () => request<Producto[]>('/productos'),
		create: (input: ProductoInput) =>
			request<Producto>('/productos', {
				method: 'POST',
				body: JSON.stringify(input),
			}),
		update: (id: number, input: ProductoInput) =>
			request<Producto>(`/productos/${id}`, {
				method: 'PUT',
				body: JSON.stringify(input),
			}),
		remove: (id: number) =>
			request<void>(`/productos/${id}`, { method: 'DELETE' }),
	},
	remitos: {
		list: () => request<Remito[]>('/remitos'),
		create: (input: RemitoInput) =>
			request<RemitoCompleto>('/remitos', {
				method: 'POST',
				body: JSON.stringify(input),
			}),
	},
};

export function formatPesos(centavos: number): string {
	return (centavos / 100).toLocaleString('es-AR', {
		style: 'currency',
		currency: 'ARS',
	});
}
