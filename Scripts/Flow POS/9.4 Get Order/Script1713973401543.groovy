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
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.model.FailureHandling
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import java.util.UUID
import groovy.json.JsonBuilder
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.model.FailureHandling
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import java.util.UUID
import groovy.json.JsonBuilder
import groovy.json.JsonSlurper

try {
// Construir el cuerpo de la solicitud con el correo electrónico aleatorio
	def orderId = GlobalVariable.orderId
			print "${orderId}"

    // URL base para las peticiones
	def urlBase = GlobalVariable.url_base + "/order/findFullOrderLastStatus?order_uuid=" +  GlobalVariable.orderId
	

    // Construir el objeto de solicitud
    def request = new com.kms.katalon.core.testobject.RestRequestObjectBuilder()
         .withRestUrl(urlBase)
        .withHttpHeaders([
          new com.kms.katalon.core.testobject.TestObjectProperty("Authorization", com.kms.katalon.core.testobject.ConditionType.EQUALS, "Bearer " + GlobalVariable.token)
        ])
        .withRestRequestMethod("GET")
        .build()


		 // Agregar impresiones para verificar la URL de la solicitud
		println "Request URL: ${request.getRestUrl()}"
	
    // Enviar la solicitud
    def response = WS.sendRequest(request, FailureHandling.CONTINUE_ON_FAILURE)


    println "Response Code: ${response.getStatusCode()}"
    println "Response: ${response.getResponseText()}"

    // Manejar la respuesta
   if (response.getStatusCode() >= 200 && response.getStatusCode() < 300){
        println "Petición enviada con éxito."
   
	// Convertir la respuesta JSON en un objeto Groovy
		def jsonResponse = response.getResponseText()
		// Parsear el JSON
		def jsonSlurper = new JsonSlurper()
		def parsedResponse = jsonSlurper.parseText(jsonResponse)
		
		// Acceder al último estado del pedido
		def orderStatus = parsedResponse.order.orderStatus[-1]


		print orderStatus
		print orderStatus.status
		if(orderStatus.status == 'closed') {
			print jsonResponse
			KeywordUtil.markPassed("El estado del pedido es cerrado (closed).")
		}else {
			 KeywordUtil.markFailed("El estado del pedido no es cerrado (closed).")
    KeywordUtil.logError("El estado actual del pedido es: $status")
		}
		
   	} else {
		println "Error al enviar la petición."
   KeywordUtil.markFailed("El caso de prueba falló debido a: Código de estado de respuesta inesperado (${response.getStatusCode()},${response.getResponseText()})")
}
} catch (Exception e) {
 println "Error al construir o enviar la solicitud: ${e.getMessage()}"
KeywordUtil.markFailed("El caso de prueba falló debido a: ${e.getMessage()}")
}
