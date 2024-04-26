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

try {
	// URL base para las peticiones
	def urlBase = GlobalVariable.url_base + '/auth/access_token'

	// Construir el objeto de solicitud
	def request = new com.kms.katalon.core.testobject.RestRequestObjectBuilder()
		.withRestUrl(urlBase)
		.withHttpHeaders([
			new com.kms.katalon.core.testobject.TestObjectProperty("Content-Type", com.kms.katalon.core.testobject.ConditionType.EQUALS, "application/json"),
			new com.kms.katalon.core.testobject.TestObjectProperty("Authorization", com.kms.katalon.core.testobject.ConditionType.EQUALS, "Bearer " + GlobalVariable.token),
			new com.kms.katalon.core.testobject.TestObjectProperty("x-api-key", com.kms.katalon.core.testobject.ConditionType.EQUALS, GlobalVariable.api_key_pos)
		])
		.withRestRequestMethod("GET")
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

	
			// Obtener el access token
			def accessToken = jsonResponse.access_token

			// Asignar el access token a la variable GlobalVariable.Depay_Token_User
			GlobalVariable.Depay_Token_User = accessToken

			println "Valor de Depay_token: ${GlobalVariable.Depay_Token_User}"
	
		
	} else {
		     println "Error al enviar la petición."
        KeywordUtil.markFailed("El caso de prueba falló debido a: Código de estado de respuesta inesperado (${response.getStatusCode()})")
	}
} catch (Exception e) {
	  println "Error al construir o enviar la solicitud: ${e.getMessage()}"
    KeywordUtil.markFailed("El caso de prueba falló debido a: ${e.getMessage()}")
}
