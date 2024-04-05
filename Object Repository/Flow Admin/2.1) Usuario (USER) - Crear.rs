<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>2.1) Usuario (USER) - Crear</name>
   <tag></tag>
   <elementGuidId>a6d3f2ec-b50f-4869-8741-064095d5bd6d</elementGuidId>
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
  &quot;text&quot;: &quot;{\n    \&quot;user\&quot;: {\n        \&quot;username\&quot;: \&quot;Test User Automatic\&quot;,\n        \&quot;password\&quot;: \&quot;1234\&quot;,\n        \&quot;role\&quot;: \&quot;user\&quot;\n    }\n   // \&quot;customer_uuid\&quot;: \&quot;019df596-d3c3-427a-aa7a-bfdb7fb1af1e\&quot; //al ser de tipo user necesita si o si estar vinculado a un customer\n    //ya que es el usuario que por ej carrefour usaria, y carrefour es un customer\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzM4NCIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImFkbWluaXN0cmFkb3IiLCJzdWIiOjI0LCJyb2xlIjoic3VwZXJfYWRtaW4iLCJ1dWlkIjoiM2I5ZjJiNTQtZTQzYS00MTBmLTkzZGMtYzQxY2MwYTI5MzhiIiwiY3VzdG9tZXJfdXVpZCI6bnVsbCwiY2xpZW50X3V1aWQiOm51bGwsImlhdCI6MTcxMTAzMTMyNywiZXhwIjoxNzExMTE3NzI3fQ.iE5OjZ5f_uE86JmGNKmq3Bsa6fLBfgJnRXa4XFrrOy-OYuvH7wtjcB2iBUdaK4G2</value>
      <webElementGuid>8d445cd2-a79c-4923-86cd-ab0bdb41f0bb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url_base}/user/create?collectorId=a37b0674-b4a4-45e9-b609-6321df5dc92c</restUrl>
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
      <id>4007f246-6dc5-4aef-8f41-a6945b766b07</id>
      <masked>false</masked>
      <name>url_base</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.token</defaultValue>
      <description></description>
      <id>ef61c808-6d24-48be-9909-bcc11e183e25</id>
      <masked>false</masked>
      <name>variable</name>
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
