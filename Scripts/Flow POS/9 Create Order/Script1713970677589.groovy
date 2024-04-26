 import groovy.json.JsonSlurper
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.model.FailureHandling
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.util.KeywordUtil
import java.util.UUID
import groovy.json.JsonBuilder
import groovy.json.JsonSlurper
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS

// Generar un UUID único
def uniqueId = UUID.randomUUID().toString()

// Extraer solo los caracteres alfanuméricos del UUID y limitarlo a 10 caracteres
def uniqueIdSubstring = uniqueId.replaceAll("[^a-zA-Z0-9]", "").substring(0, 10)

println "uniqueIdSubstring: ${uniqueIdSubstring}"
println "uniqueId: ${uniqueId}"


// Construir el cuerpo de la solicitud con el correo electrónico aleatorio
def requestBody = """
{
    "external_reference": "${uniqueIdSubstring}",
    "total_amount": 1000,
    "description": "Compra en HASAR",
    "title": "Compra en HASAR",
    "notification_url": "https://typedwebhook.tools/webhook/fab28b4a-60d2-41e5-b3b5-e90bf9cfc903?source_news=ipn",
   
    "items": [
        {
            "sku_number": "serie",
            "category": "category",
            "title": "title",
            "description": "description",
            "quantity": 3,
            "unit_measure": "unit",
            "unit_price": "500",
            "total_amount": 1500,
            "currency_id": "USDT"
        },
                {
            "sku_number": "serie 2",
            "category": "category",
            "title": "title",
            "description": "description",
            "quantity": 3,
            "unit_measure": "unit",
            "unit_price": "500",
            "total_amount": 1500,
            "currency_id": "USDT"
        }
    ]
}
"""

try {
	// Obtener el valor de GlobalVariable.customer_uuid
	def customer_uuid = GlobalVariable.external_Customer
	// Obtener el valor de GlobalVariable.customer_uuid
	def store_id = GlobalVariable.external_pos
	// Obtener el valor de GlobalVariable.customer_uuid
	def access_token = GlobalVariable.Depay_Token_User
    // URL base para las peticiones
	def urlBase = GlobalVariable.url_base + "/instore/qr/seller/collectors/${customer_uuid}/pos/${store_id}/orders?access_token=${access_token}"



    // Construir el objeto de solicitud
    def request = new com.kms.katalon.core.testobject.RestRequestObjectBuilder()
         .withRestUrl(urlBase)
        .withHttpHeaders([
            new com.kms.katalon.core.testobject.TestObjectProperty("Content-Type", com.kms.katalon.core.testobject.ConditionType.EQUALS, "application/json"),
            new com.kms.katalon.core.testobject.TestObjectProperty("Authorization", com.kms.katalon.core.testobject.ConditionType.EQUALS, "Bearer " + GlobalVariable.token_user)
        ])
        .withTextBodyContent(requestBody)
        .withRestRequestMethod("POST")
        .build()


		 // Agregar impresiones para verificar la URL de la solicitud
		println "Request URL: ${request.getRestUrl()}"
		
	

	// Agregar impresiones para verificar el cuerpo de la solicitud
	println "Request Body: ${request.getBodyContent()}"
    // Enviar la solicitud
    def response = WS.sendRequest(request, FailureHandling.CONTINUE_ON_FAILURE)

  println "Response Code: ${response.getStatusCode()}"
    println "Response: ${response.getResponseText()}"

    // Manejar la respuesta
    if (response.getStatusCode() >= 200 && response.getStatusCode() < 300) {
        println "Petición enviada con éxito."
   
		// Convertir la respuesta JSON en un objeto Groovy
		def jsonResponse = new JsonSlurper().parseText(response.getResponseText())
		print jsonResponse

		GlobalVariable.orderId = jsonResponse.data.uuid
  	} else {
		     println "Error al enviar la petición."
        KeywordUtil.markFailed("El caso de prueba falló debido a: Código de estado de respuesta inesperado (${response.getStatusCode()})")
	}
} catch (Exception e) {
	  println "Error al construir o enviar la solicitud: ${e.getMessage()}"
    KeywordUtil.markFailed("El caso de prueba falló debido a: ${e.getMessage()}")
}

