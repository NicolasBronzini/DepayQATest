import groovy.json.JsonSlurper
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.model.FailureHandling
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import java.util.UUID
import groovy.json.JsonBuilder


try {
    // URL base para la petición GET
    def urlBase = GlobalVariable.url_base + '/auth/access_token'

    // Construir el objeto de solicitud
    def request = new com.kms.katalon.core.testobject.RestRequestObjectBuilder()
        .withRestUrl(urlBase)
        .withHttpHeaders([
            new com.kms.katalon.core.testobject.TestObjectProperty("api_key", com.kms.katalon.core.testobject.ConditionType.EQUALS, GlobalVariable.api_key_user)
        ])
        .withRestRequestMethod("GET")
        .build()

    // Agregar impresiones para verificar la URL de la solicitud
    println "Request URL: ${request.getRestUrl()}"

    // Enviar la solicitud
    def response = WS.sendRequest(request, FailureHandling.CONTINUE_ON_FAILURE)

    println "Response: ${response.getResponseText()}"

    // Manejar la respuesta
    if (response) {
        println "Petición enviada con éxito."
        // Puedes manejar la respuesta según tus necesidades aquí
		
		// Convertir la respuesta JSON en un objeto Groovy
		def jsonResponse = new JsonSlurper().parseText(response.getResponseText())
		print jsonResponse
    } else {
        println "Error al enviar la petición."
    }
} catch (Exception e) {
    println "Error al construir o enviar la solicitud: ${e.getMessage()}"
}
