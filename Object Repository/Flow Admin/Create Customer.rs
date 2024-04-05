<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create Customer</name>
   <tag></tag>
   <elementGuidId>af28f7b4-b1ba-449e-925a-a6c165df89c5</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${token}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;Mayorista Maxi\&quot;, \n  \&quot;email\&quot;: \&quot;tests2222@gmail.com\&quot;,\n  \&quot;address\&quot;: \&quot;Av Marce\&quot;,\n  \&quot;phone\&quot;: \&quot;1148784215\&quot;,\n  \&quot;company\&quot;: \&quot;Cliente SA\&quot;,\n  \&quot;external_reference\&quot;: \&quot;Mayorista\&quot;,\n  \&quot;description\&quot;: \&quot;Branch Example\&quot;,\n  \&quot;manager\&quot;: \&quot;Manager\&quot;,\n  \&quot;wallet\&quot;: {\n      \&quot;address\&quot;: \&quot;0x71C7656EC7ab88b098defB751B7401B5f6d8976F\&quot;,\n      \&quot;network\&quot;: \&quot;Ethereum\&quot;,\n      \&quot;coin\&quot;: \&quot;ETH\&quot;\n  }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>7c71bbfe-253d-4f9a-9084-5bad3b61796c</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url_base}/customer</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url_base</defaultValue>
      <description></description>
      <id>b95bf9c5-77ed-44e1-900c-2185edbeffae</id>
      <masked>false</masked>
      <name>url_base</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>5f64fe02-9aab-4cdb-9ec5-cd0e2d3cd4b0</id>
      <masked>false</masked>
      <name>token</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
