import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WS.sendRequest(findTestObject('Flow Admin/3) Store - Crear sin POS', [('url_base') : GlobalVariable.url_base]))

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
    "user": {
		"username": "QATest${uniqueIdSubstring}",
        "password": "1234",
        "role": "user"
    }
}
"""

try {
    // Obtener el valor de GlobalVariable.customer_uuid
    def customerUuid = GlobalVariable.customer_uuid

    // URL base para las peticiones
    def urlBase = GlobalVariable.url_base + '/store?collectorId=' + customerUuid
	
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

		GlobalVariable.api_key_user = jsonResponse.data.apiKey
    } else {
        println "Error al enviar la petición."
    }
} catch (Exception e) {
    println "Error al construir o enviar la solicitud: ${e.getMessage()}"
}
