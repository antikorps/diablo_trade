let informacionDesplegables = {}
const $desplegableAgrupamiento = document.querySelector("#objeto-agrupamiento");
const $objetoNombreContenedor = document.querySelector("#objeto-nombre-contenedor")
const $objetoNombre = document.querySelector("#objeto-nombre")

function prepararDesplegables() {
    const agrupamientoUnaMano = ["axe", "dagger", "mace", "scythe", "sword", "wand"]
    const agrupamientoDosManos = ["polearm", "staff", "twohandedaxe", "twohandedmace", "twohandedscythe", "twohandedsword"];
    const agrupamientoDistancia = ["bow", "crossbow"];

    const agrupamientosTotales = {
        "unaMano": {
            "total": 0,
            "objetos": []
        },
        "dosManos": {
            "total": 0,
            "objetos": []
        },
        "distancia": {
            "total": 0,
            "objetos": []
        }
    }

    for (const r of informe["info"]) {
        const tipo = r["original"]["itemType"]
        const nombre = r["original"]["name"]
        const similares = r["similares"].length
        if (agrupamientoUnaMano.includes(tipo)) {
            agrupamientosTotales["unaMano"]["total"]++
            if (agrupamientosTotales["unaMano"]["objetos"].hasOwnProperty(nombre)) {
                console.warn(`nombre de objeto repetido ${nombre}`)
            } else {
                agrupamientosTotales["unaMano"]["objetos"].push({
                    "nombre": nombre,
                    "similares": similares
                })
                continue
            }
        }
        if (agrupamientoDosManos.includes(tipo)) {
            agrupamientosTotales["dosManos"]["total"]++
            if (agrupamientosTotales["dosManos"]["objetos"].hasOwnProperty(nombre)) {
                console.warn(`nombre de objeto repetido ${nombre}`)
            } else {
                agrupamientosTotales["dosManos"]["objetos"].push({
                    "nombre": nombre,
                    "similares": similares
                })
                continue
            }
        }
        if (agrupamientoDistancia.includes(tipo)) {
            agrupamientosTotales["distancia"]["total"]++
            if (agrupamientosTotales["distancia"]["objetos"].hasOwnProperty(nombre)) {
                console.warn(`nombre de objeto repetido ${nombre}`)
            } else {
                agrupamientosTotales["distancia"]["objetos"].push({
                    "nombre": nombre,
                    "similares": similares
                })
                continue
            }
        }

        // No agrupamientos
        if (agrupamientosTotales.hasOwnProperty(tipo)) {
            agrupamientosTotales[tipo]["total"] += 1
            agrupamientosTotales[tipo]["objetos"].push({
                "nombre": nombre,
                "similares": similares
            })

        } else {
            agrupamientosTotales[tipo] = {
                "total": 0,
                "objetos": []
            }

            agrupamientosTotales[tipo]["total"] += 1
            agrupamientosTotales[tipo]["objetos"].push({
                "nombre": nombre,
                "similares": similares
            })
        }
    }

    // Ordenar alfabeticamente objetos
    let agrupamientosObjetosOrdenados = {}

    for (const clave in agrupamientosTotales) {
        const agrupamiento = agrupamientosTotales[clave]
        const objetos = agrupamiento["objetos"].sort((a, b) => a.nombre.localeCompare(b.nombre));

        agrupamientosObjetosOrdenados[clave] = {
            "total": agrupamientosTotales[clave]["total"],
            "objetos": objetos
        }
    }
    informacionDesplegables = agrupamientosObjetosOrdenados
}

function incorporarOpcionesAgrupamiento() {
    let opciones = `   
    <option value="unaMano">Una mano (${informacionDesplegables["unaMano"]["total"]})</option>
    <option value="dosManos">Dos manos (${informacionDesplegables["dosManos"]["total"]})</option>
    <option value="distancia">Distancia (${informacionDesplegables["distancia"]["total"]})</option>
    `
    // Listar alfabeticamente
    const clavesOrdenadas = Object.keys(informacionDesplegables).sort()
    for (const tipo of clavesOrdenadas) {
        if (tipo == "unaMano" || tipo == "dosManos" || tipo == "distancia") {
            continue
        }
        opciones += `<option value="${tipo}">${tipo} (${informacionDesplegables[tipo]["total"]})</option>`
    }

    $desplegableAgrupamiento.insertAdjacentHTML("beforeend", opciones)

    $desplegableAgrupamiento.addEventListener("change", actualizarNombreObjeto)
}

function incorporarOpcionesObjeto(agrupamiento) {
    let opciones = `<option value=""></option>`

    for (const objeto of informacionDesplegables[agrupamiento]["objetos"]) {
        opciones += `<option value="${objeto["nombre"]}">${objeto["nombre"]} (${objeto["similares"]})</option>`
    }

    $objetoNombre.innerHTML = opciones
    $objetoNombre.addEventListener("change", mostrarObjetoSimilares)

}

function actualizarNombreObjeto(evento) {
    let agrupamiento = evento.target.value
    if (agrupamiento == "") {
        $objetoNombreContenedor.classList.add("oculto")
        return
    }
    incorporarOpcionesObjeto(agrupamiento)
    $objetoNombreContenedor.classList.remove("oculto")
}

const $resultados = document.querySelector("#resultados")
const $comparacionObjeto = document.querySelector("#comparacion-objeto")
const $comparacionSeleccionado = document.querySelector("#comparacion-seleccionado")
const $comparacionPosibilidades = document.querySelector("#comparacion-posibilidades")

function iniciarApp() {
    prepararDesplegables()
    incorporarOpcionesAgrupamiento()
}

function actualizarObjetoNombre(evento) {
    $comparacionObjeto.innerHTML = ""
    $comparacionSeleccionado.innerHTML = ""
    $comparacionPosibilidades.innerHTML = ""
    $resultados.innerHTML = ""

    $objetoNombreContenedor.classList.add("oculto")

    const valorSeleccionado = evento.target.value
    if (valorSeleccionado == "") {
        return
    }

    let opcionesNombre = `<option value=""></option>`
    const nombresRecopilados = []
    for (const nombre of tiposObjetos[valorSeleccionado]) {
        nombresRecopilados.push(nombre)
    }
    nombresRecopilados.sort()
    for (const nombreRecopilado of nombresRecopilados) {
        opcionesNombre += `<option value="${nombreRecopilado}">${nombreRecopilado} (${tipoSimilares[nombreRecopilado]})</option>`
    }

    $objetoNombre.innerHTML = opcionesNombre

    $objetoNombreContenedor.classList.remove("oculto")
}

function relacionarAfijos(objeto, similar) {
    const idsComunes = []
    const objetoDiferentes = []
    const similarDiferentes = []

    for (const id of similar) {
        if (objeto.includes(id)) {
            idsComunes.push(id)
        } else {
            similarDiferentes.push(id)
        }
    }

    for (const id of objeto) {
        if (!similar.includes(id)) {
            objetoDiferentes.push(id)
        }
    }

    return [idsComunes, objetoDiferentes, similarDiferentes]
}

function compararObjeto(evento) {
    let identificador = evento.target.value.split("-")

    let objetoIndice = parseInt(identificador[0])
    let similarIndice = parseInt(identificador[1])

    const objetoInfo = informe["info"][objetoIndice]["original"]
    const similarInfo = informe["info"][objetoIndice]["similares"][similarIndice]

    const objetoTipo = objetoInfo["itemType"]
    const similarTipo = similarInfo["itemType"]
    const similarPrecio = similarInfo["price"].toLocaleString()

    // IMPLICITOS
    let tablaCuerpo = "<tbody>"

    try {
        const objetoImplicito = objetoInfo["implicits"][0]
        const objetoImplicitoId = objetoImplicito["id"]
        const objetoImplicitoValor = objetoImplicito["value"] // Recuperar
        const objetoImplicitoNormalizado = relaciones[objetoTipo]["implicits"][objetoImplicitoId]["name"] // Recuperar

        const similarImplicito = similarInfo["implicits"][0]

        const similarImplicitoId = similarImplicito["id"]
        const similarImplicitoValor = similarImplicito["value"] // Recuperar
        const similarImplicitoNormalizado = relaciones[similarTipo]["implicits"][similarImplicitoId]["name"] // Recuperar

        tablaCuerpo += `<tr class="implicito"><td>${objetoImplicitoNormalizado}</td><td>${objetoImplicitoValor} %</td><td>${similarImplicitoValor} %</td><td>${similarImplicitoNormalizado}</td>`
    } catch (error) {
        tablaCuerpo += `<tr class="implicito"><td colspan="4">Sin información sobre el implícito</td></tr>`
    }

    // AFIJOS
    const objetoAfijos = objetoInfo["affixes"]
    const objetoAfijosId = []
    for (const a of objetoAfijos) {
        objetoAfijosId.push(a["id"])
    }

    const similarAfijos = similarInfo["affixes"]
    const similarAfijosId = []
    for (const s of similarAfijos) {
        similarAfijosId.push(s["id"])
    }

    const afijosIdsRelacionados = relacionarAfijos(objetoAfijosId, similarAfijosId)
    const afijosComunes = afijosIdsRelacionados[0]
    const afijosObjetoDiferentes = afijosIdsRelacionados[1]
    const afijosSimilarDiferentes = afijosIdsRelacionados[2]
    // TABULAR AFIJOS COMUNES
    for (const comun of afijosComunes) {
        // Objeto
        let objetoAfijoNombreNormalizado = ""
        let objetoAfijoValor = ""

        for (const a of objetoAfijos) {
            const afijoId = a["id"]
            if (afijoId != comun) {
                continue
            }

            objetoAfijoValor = a["value"] // Recuperar
            objetoAfijoNombreNormalizado = relaciones[objetoTipo]["affixes"][afijoId]["name"] // Recuperar
            break
        }
        // Similar
        let similarAfijoNombreNormalizado = ""
        let similarAfijoValor = ""
        for (const a of similarAfijos) {
            const afijoId = a["id"]
            if (afijoId != comun) {
                continue
            }

            similarAfijoValor = a["value"] // Recuperar
            similarAfijoNombreNormalizado = relaciones[similarTipo]["affixes"][afijoId]["name"] // Recuperar
            break
        }

        tablaCuerpo += `<tr class="afijo-comun"><td>${objetoAfijoNombreNormalizado}</td><td>${objetoAfijoValor} %</td><td>${similarAfijoValor} %</td><td>${similarAfijoNombreNormalizado}</td></tr>`
    }

    // TABULAR AFIJOS DISTINTOS
    for (let i = 0; i < afijosObjetoDiferentes.length; i++) {
        const objetoIdDiferente = afijosObjetoDiferentes[i]
        const objetoNombreNormalizadoDiferente = relaciones[objetoTipo]["affixes"][objetoIdDiferente]["name"]
        let objetoValorDiferente = ""
        for (const a of objetoAfijos) {
            const aId = a["id"]
            if (aId != objetoIdDiferente) {
                continue
            }
            objetoValorDiferente = a["value"]
        }

        const similarIdDiferente = afijosSimilarDiferentes[i]
        const similarNombreNormalizadoDiferente = relaciones[similarTipo]["affixes"][similarIdDiferente]["name"]
        let similarValorDiferente = ""
        for (const a of similarAfijos) {
            const aId = a["id"]
            if (aId != similarIdDiferente) {
                continue
            }
            similarValorDiferente = a["value"]
        }

        tablaCuerpo += `<tr class="afijo-distinto"><td>${objetoNombreNormalizadoDiferente}</td><td>${objetoValorDiferente} %</td><td>${similarValorDiferente} %</td><td>${similarNombreNormalizadoDiferente}</td></tr>`

    }
    tablaCuerpo += "</tbody>"
    const tabla = `
    
    <table>
        <thead>
            <th colspan="2"><p>${objetoInfo["name"]}</p><p>${objetoInfo["itemType"]}</p></th>
            <th colspan="2"><p>${similarInfo["name"]} (${similarPrecio} €)</p><p>${similarInfo["itemType"]}</p></th>
        </thead>
        ${tablaCuerpo}
    </table>
    `

    $resultados.innerHTML = tabla

    const imagen = similarInfo["image"]
    const imagenUrl = "https://diablo-trade-images.nyc3.digitaloceanspaces.com/" + imagen;

    const similarNombre = similarInfo["name"]
    $comparacionSeleccionado.innerHTML = `<p><img src="${imagenUrl}" alt="${similarNombre}" title="${similarNombre}"></p>`

}

function mostrarObjetoSimilares(evento) {
    $comparacionObjeto.innerHTML = ""
    $comparacionSeleccionado.innerHTML = ""
    $comparacionPosibilidades.innerHTML = ""
    $resultados.innerHTML = ""

    const nombreObjeto = evento.target.value

    for (const [indice, registro] of informe["info"].entries()) {
        if (registro["original"]["name"] == nombreObjeto) {
            // Pintar la imagen 
            const imagen = registro["original"]["image"]
            const imagenUrl = "https://diablo-trade-images.nyc3.digitaloceanspaces.com/" + imagen;

            $comparacionObjeto.innerHTML = `<p><img src="${imagenUrl}" alt="${nombreObjeto}" title="${nombreObjeto}"></p>`
            if (registro["similares"].length == 0) {
                $comparacionPosibilidades.innerHTML = "<p>No se ha encontrado ningún objeto similar</p>"
                return
            }
            // Buscar similares
            let recopilacionSimilares = "<fieldset>"
            for (const [i, s] of registro["similares"].entries()) {
                const nombreSimilar = s["name"]
                const precioSimilar = s["price"].toLocaleString()

                recopilacionSimilares += `
                <label>
                    <input type="radio" name="similar" value="${indice}-${i}">${nombreSimilar} - ${precioSimilar} €
                </label>`
            }
            recopilacionSimilares += "</fielset>"
            $comparacionPosibilidades.innerHTML = recopilacionSimilares

            const selectoresSimilares = document.querySelectorAll("input[name='similar']")

            for (const s of selectoresSimilares) {
                s.addEventListener("change", compararObjeto)
            }
            break;
        }
    }
}

document.addEventListener("DOMContentLoaded", iniciarApp)