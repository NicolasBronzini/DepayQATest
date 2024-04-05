import groovy.json.JsonSlurper
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.testobject.HttpBodyContent
import com.kms.katalon.core.model.FailureHandling
import java.io.File
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository

try {
    // Lee el contenido del archivo JSON
    def jsonString = new File('wallets.json').text
    
    // Parsea el contenido JSON
    def walletsData = new JsonSlurper().parseText(jsonString)
    
    // Imprime el contenido del archivo JSON
    println "Contenido del archivo JSON cargado correctamente:"
    println walletsData
    
    // URL base para las peticiones
    def urlBase = GlobalVariable.url_base + '/wallet'
    
    // Obtener el objeto de prueba 'Create Wallet'
    def testObject = ObjectRepository.findTestObject('Create Wallet')
    
		walletsData.each { wallet ->
    if (wallet.text) {
        // Construir el cuerpo de la solicitud
        def requestBody = """
        {
          "wallet": {
            "code": "${wallet.text}",
            "description": "${wallet.text}"
          },
          "tokenCodes": ["USDT", "USDC", "DAI", "ETH"],
		  "networksCodes": ["TRON","Ethereum","BNB","Polygon","Polygon","Arbitrum" ]
        }
        """ 
  	print requestBody
				
      try {
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
		    def response = WS.sendRequestAndVerify(request, FailureHandling.CONTINUE_ON_FAILURE)
		
		    // Manejar la respuesta
		    if (response) {
		        println "Petición enviada con éxito."
		        println response
		    } else {
		        println "Error al enviar la petición."
				println response
		    }
		} catch (Exception e) {
		    println "Error al construir o enviar la solicitud: ${e.getMessage()}"
		}

    	} 
     
    }
} catch (Exception e) {
    // Maneja cualquier error que ocurra al cargar el archivo JSON
    println "Error al cargar el archivo JSON: ${e.message}"
}
