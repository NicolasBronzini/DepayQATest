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
  "name": "QATestPOS",
  "email": "QATest${uniqueIdSubstring}@gmail.com",
  "address": "QATest",
  "phone": "1148784215",
  "company": "QATest",
  "external_reference": "QATest${uniqueIdSubstring}",
"depay_fee":"0.04",
  "description": "QATest",
  "manager": "QATest",
  "wallet": {
      "address": "0x71C7656EC7ab88b098defB751B7401B5f6d8976F",
      "network": "Ethereum",
      "coin": "ETH"
  }
}
"""

try {
	// Obtener el valor de GlobalVariable.customer_uuid
	def client_uuid = GlobalVariable.client_uuid
    // URL base para las peticiones
	def urlBase = GlobalVariable.url_base + '/customer?client_uuid=' + client_uuid



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
	GlobalVariable.customer_uuid = jsonResponse.data.uuid
	
	GlobalVariable.external_Customer=jsonResponse.data.external_reference
		
  	} else {
		     println "Error al enviar la petición."
        KeywordUtil.markFailed("El caso de prueba falló debido a: Código de estado de respuesta inesperado (${response.getStatusCode()})")
	}
} catch (Exception e) {
	  println "Error al construir o enviar la solicitud: ${e.getMessage()}"
    KeywordUtil.markFailed("El caso de prueba falló debido a: ${e.getMessage()}")
}

