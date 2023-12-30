fn devolver_css() -> String {
    let archivo_css = include_str!("estilos.css");
    let css_min = archivo_css.replace("\n", "");

    return css_min.to_string();
}

fn devolver_javascript() -> String {
    let archivo_js = include_str!("scripts.js");
    return archivo_js.to_string();
}

fn devolver_relaciones() -> String {
    let archivo_relaciones = include_str!("relaciones.json");
    return archivo_relaciones.to_string();
}

pub fn generar_html(informe: String) -> String {
    let css = devolver_css();
    let javascript = devolver_javascript();
    let relaciones = devolver_relaciones();

    return format!(
        r###"    
<!DOCTYPE html>
<html lang="es">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="stylesheet" href="milligram.min.css">
    <title>Informe</title>
    <style>{css}</style>
</head>


<body>
<div class="container">
<div class="row">
    
    <div class="column">
        <form id="formulario">
            <fieldset>
                <label>Selecciona el agrupamiento del objeto:</label>
                <select id="objeto-agrupamiento">
                    <option value=""></option>
                </select>
            </fieldset>
            <fieldset id="objeto-nombre-contenedor" class="oculto">
                <label>Selecciona el nombre del objeto:</label>
                <select id="objeto-nombre">
                </select>
            </fieldset>
        </form>
    </div>

    <div class="column">
        <img id="logo-diablo" src="https://diablo.trade/_next/image?url=%2Fsanctuary-logo.png&amp;w=256&amp;q=75">
    </div>

</div>

    <div class="row" id="resultados">
    </div>

    <div class="row" id="comparacion">
        <div class="column" id="comparacion-objeto">

        </div>
        <div class="column" id="comparacion-seleccionado">

        </div>

        <div class="column" id="comparacion-posibilidades">

        </div>

    </div>
</div>

<script>
    const informe = {informe}
    const relaciones = {relaciones}

    {javascript}
</script>
    
</body>
</html>"###
    );
}
