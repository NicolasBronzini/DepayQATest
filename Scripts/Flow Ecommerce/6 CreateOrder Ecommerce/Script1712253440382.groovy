import groovy.json.JsonSlurper
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.model.FailureHandling
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import java.util.UUID
import groovy.json.JsonBuilder



// Generar un UUID único
def uniqueId = UUID.randomUUID().toString()

// Extraer solo los caracteres alfanuméricos del UUID y limitarlo a 10 caracteres
def uniqueIdSubstring = uniqueId.replaceAll("[^a-zA-Z0-9]", "").substring(0, 10)

println "uniqueIdSubstring: ${uniqueIdSubstring}"
println "uniqueId: ${uniqueId}"

// Construir el cuerpo de la solicitud con el correo electrónico aleatorio
def requestBody = """
{

   "order": {
        "external_reference": "QATest",
        "total_amount": 0.01,
        "description": "QATest",
        "title": "QATest",
        "notification_url": "https://zqmh0a9jg3.execute-api.sa-east-1.amazonaws.com/Prod/listenorder"
    },
    "items": [
        {
            "sku_number": "QATest",
            "category": "QATest",
            "title": "QATest",
            "description": "QATest",
            "quantity": 3,
            "unit_measure": "unit",
            "unit_price": "500",
            "total_amount": 1,
            "currency_id": "USDT"
        }
    ]
}
"""

try {
	// Obtener el valor de GlobalVariable.customer_uuid
	def customerUUID = GlobalVariable.customer_uuid
    // URL base para las peticiones
	def urlBase = GlobalVariable.url_base + '/order/create/ecommerce?collectorId=bc80a833-c4af-4670-98bf-c0e245e7f85a'



    // Construir el objeto de solicitud
    def request = new com.kms.katalon.core.testobject.RestRequestObjectBuilder()
         .withRestUrl(urlBase)
        .withHttpHeaders([
            new com.kms.katalon.core.testobject.TestObjectProperty("Content-Type", com.kms.katalon.core.testobject.ConditionType.EQUALS, "application/json"),
            new com.kms.katalon.core.testobject.TestObjectProperty("Authorization", com.kms.katalon.core.testobject.ConditionType.EQUALS, "Bearer " + GlobalVariable.token)
        ])
        .withTextBodyContent(requestBody)
        .withRestRequestMethod("POST")
        .build()


		 // Agregar impresiones para verificar la URL de la solicitud
		println "Request URL: ${request.getRestUrl()}"
		println "Request Body: ${requestBody}"
	

	// Agregar impresiones para verificar el cuerpo de la solicitud
	println "Request Body: ${request.getBodyContent()}"
    // Enviar la solicitud
    def response = WS.sendRequest(request, FailureHandling.CONTINUE_ON_FAILURE)

  println "Response: ${response.getResponseText()}"

    // Manejar la respuesta
    if (response) {
        println "Petición enviada con éxito."
   
		// Convertir la respuesta JSON en un objeto Groovy
		def jsonResponse = new JsonSlurper().parseText(response.getResponseText())
		print jsonResponse

		
    } else {
        println "Error al enviar la petición."
    }
} catch (Exception e) {
    println "Error al construir o enviar la solicitud: ${e.getMessage()}"
}
