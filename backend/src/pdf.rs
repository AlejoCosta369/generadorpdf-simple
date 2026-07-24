use crate::models::{Cliente, Empresa, RemitoCompleto};
use printpdf::{Base64OrRaw, GeneratePdfOptions, PdfDocument, PdfSaveOptions};
use std::collections::BTreeMap;

fn format_centavos(centavos: i64) -> String {
    format!("{}.{:02}", centavos / 100, (centavos % 100).abs())
}

fn render_html(empresa: &Empresa, cliente: &Cliente, remito: &RemitoCompleto) -> String {
    let fecha = &remito.remito.fecha;
    let numero = format!("{:06}", remito.remito.id);

    let mut rows = String::new();
    for item in &remito.items {
        rows.push_str(&format!(
            r#"<tr>
                <td class="cell">{nombre}</td>
                <td class="cell right">{cantidad}</td>
                <td class="cell right">${precio}</td>
                <td class="cell right">${subtotal}</td>
            </tr>"#,
            nombre = item.nombre_producto,
            cantidad = item.cantidad,
            precio = format_centavos(item.precio_unitario_centavos),
            subtotal = format_centavos(item.subtotal_centavos),
        ));
    }

    format!(
        r#"<html>
<head>
<style>
    body {{ font-family: Helvetica, Arial, sans-serif; font-size: 12px; color: #222; padding: 24px; }}
    .header {{ font-size: 22px; font-weight: bold; margin-bottom: 4px; }}
    .subheader {{ font-size: 13px; color: #555; margin-bottom: 20px; }}
    .info-flex {{ display: flex; justify-content: space-between; margin-bottom: 20px; }}
    .info-col {{ width: 46%; }}
    .left-col {{ border-right: 1px solid #999999; padding-right: 16px; }}
    .section-title {{ font-weight: bold; font-size: 13px; margin-bottom: 6px; }}
    .info-row {{ margin-bottom: 3px; }}
    .label {{ font-weight: bold; }}
    table {{ width: 100%; border-collapse: collapse; margin-top: 10px; }}
    .cell {{ border: 1px solid #ccc; padding: 6px; font-size: 12px; }}
    .head-cell {{ border: 1px solid #ccc; padding: 6px; font-size: 12px; font-weight: bold; background-color: #eeeeee; }}
    .right {{ text-align: right; }}
    .total-row {{ font-weight: bold; font-size: 14px; margin-top: 14px; text-align: right; }}
</style>
</head>
<body>
    <div class="header">Remito N° {numero}</div>
    <div class="subheader">Fecha: {fecha}</div>

    <div class="info-flex">
        <div class="info-col left-col">
            <div class="section-title">Datos de la empresa</div>
            <div class="info-row"><span class="label">Nombre:</span> {empresa_nombre}</div>
            <div class="info-row"><span class="label">Direccion:</span> {empresa_direccion}</div>
            <div class="info-row"><span class="label">CUIT:</span> {empresa_cuit}</div>
            <div class="info-row"><span class="label">Telefono:</span> {empresa_telefono}</div>
        </div>
        <div class="info-col">
            <div class="section-title">Datos del cliente</div>
            <div class="info-row"><span class="label">Nombre:</span> {cliente_nombre}</div>
            <div class="info-row"><span class="label">Direccion:</span> {cliente_direccion}</div>
            <div class="info-row"><span class="label">CUIT/DNI:</span> {cliente_cuit}</div>
            <div class="info-row"><span class="label">Telefono:</span> {cliente_telefono}</div>
        </div>
    </div>

    <table>
        <tr>
            <td class="head-cell">Producto</td>
            <td class="head-cell right">Cantidad</td>
            <td class="head-cell right">Precio unit.</td>
            <td class="head-cell right">Subtotal</td>
        </tr>
        {rows}
    </table>

    <div class="total-row">Total: ${total}</div>
</body>
</html>"#,
        numero = numero,
        fecha = fecha,
        empresa_nombre = empresa.nombre,
        empresa_direccion = empresa.direccion,
        empresa_cuit = empresa.cuit,
        empresa_telefono = empresa.telefono,
        cliente_nombre = cliente.nombre,
        cliente_direccion = cliente.direccion,
        cliente_cuit = cliente.cuit_dni,
        cliente_telefono = cliente.telefono,
        rows = rows,
        total = format_centavos(remito.remito.total_centavos),
    )
}

pub fn generate_remito_pdf(
    empresa: &Empresa,
    cliente: &Cliente,
    remito: &RemitoCompleto,
) -> Result<Vec<u8>, String> {
    let html = render_html(empresa, cliente, remito);

    let images: BTreeMap<String, Base64OrRaw> = BTreeMap::new();
    let fonts: BTreeMap<String, Base64OrRaw> = BTreeMap::new();
    let options = GeneratePdfOptions::default();
    let mut warnings = Vec::new();

    let doc = PdfDocument::from_html(&html, &images, &fonts, &options, &mut warnings)?;

    let mut save_warnings = Vec::new();
    Ok(doc.save(&PdfSaveOptions::default(), &mut save_warnings))
}
