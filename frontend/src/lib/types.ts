export interface Cliente {
	id: number;
	nombre: string;
	direccion: string;
	cuit_dni: string;
	telefono: string;
	created_at: string;
	updated_at: string;
}

export interface ClienteInput {
	nombre: string;
	direccion: string;
	cuit_dni: string;
	telefono: string;
}

export interface Empresa {
	id: number;
	nombre: string;
	direccion: string;
	cuit: string;
	telefono: string;
	updated_at: string;
}

export interface EmpresaInput {
	nombre: string;
	direccion: string;
	cuit: string;
	telefono: string;
}

export interface Producto {
	id: number;
	nombre: string;
	precio_centavos: number;
	stock: number | null;
	created_at: string;
	updated_at: string;
}

export interface ProductoInput {
	nombre: string;
	precio_centavos: number;
	stock: number | null;
}

export interface Remito {
	id: number;
	cliente_id: number;
	fecha: string;
	total_centavos: number;
	created_at: string;
}

export interface RemitoItem {
	id: number;
	remito_id: number;
	producto_id: number | null;
	nombre_producto: string;
	precio_unitario_centavos: number;
	cantidad: number;
	subtotal_centavos: number;
}

export interface RemitoCompleto {
	remito: Remito;
	items: RemitoItem[];
}

export interface RemitoItemInput {
	producto_id: number;
	cantidad: number;
}

export interface RemitoInput {
	cliente_id: number;
	items: RemitoItemInput[];
}
