<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>1) Collector - Crear</name>
   <tag></tag>
   <elementGuidId>6cfc47d8-db73-441a-8a83-385efc6f29e2</elementGuidId>
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
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;Mayorista Maxi\&quot;,\n  \&quot;email\&quot;: \&quot;3@gmail.com\&quot;,\n  \&quot;address\&quot;: \&quot;Av Marce\&quot;,\n  \&quot;phone\&quot;: \&quot;1148784215\&quot;,\n  \&quot;company\&quot;: \&quot;Cliente SA\&quot;,\n  \&quot;external_reference\&quot;: \&quot;Mayorista\&quot;,\n  \&quot;description\&quot;: \&quot;Branch Example\&quot;,\n  \&quot;manager\&quot;: \&quot;Manager\&quot;,\n  \&quot;wallet\&quot;: {\n      \&quot;address\&quot;: \&quot;0x71C7656EC7ab88b098defB751B7401B5f6d8976F\&quot;,\n      \&quot;network\&quot;: \&quot;Ethereum\&quot;,\n      \&quot;coin\&quot;: \&quot;ETH\&quot;\n  }\n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>f5177784-bf79-46c4-b07f-f40957b56c12</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${token}</value>
      <webElementGuid>3710a2b2-c456-492b-b88d-e68bfb4169cf</webElementGuid>
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
   <validationSteps>
      <id>f764ac9a-cc5e-4ffb-9856-59bbbe1fd5e3</id>
      <name>Bearer_token</name>
      <type>JSON_SCHEMA</type>
      <dataType>AUTO</dataType>
      <target>REQUEST</target>
      <data></data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>GlobalVariable.url_base</defaultValue>
      <description></description>
      <id>a8bca370-2abb-42da-b4c9-8941cea90eb2</id>
      <masked>false</masked>
      <name>url_base</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>cca5f278-a1f8-4ab8-af6b-c79a1420321a</id>
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
