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



// Construir el cuerpo de la solicitud con el correo electrónico aleatorio
def requestBody = """
{
"store": {
        "description": "TestQA",
        "external_reference": "TEST QA",
        "address": "TestQA",
        "phone": "11111111111",
        "manager": "TestQA"
    }
}
"""

try {
	// Obtener el valor de GlobalVariable.customer_uuid
	def customer_uuid = GlobalVariable.customer_uuid
    // URL base para las peticiones
	def urlBase = GlobalVariable.url_base + '/store?collectorId=' + customer_uuid



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
	GlobalVariable.store_uuid = jsonResponse.data.uuid
		
  	} else {
		     println "Error al enviar la petición."
        KeywordUtil.markFailed("El caso de prueba falló debido a: Código de estado de respuesta inesperado (${response.getStatusCode()})")
	}
} catch (Exception e) {
	  println "Error al construir o enviar la solicitud: ${e.getMessage()}"
    KeywordUtil.markFailed("El caso de prueba falló debido a: ${e.getMessage()}")
}

