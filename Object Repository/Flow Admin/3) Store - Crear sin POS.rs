<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>3) Store - Crear sin POS</name>
   <tag></tag>
   <elementGuidId>e28da6a0-72e2-41d3-ab4e-0197176f2816</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzM4NCIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImFkbWluaXN0cmFkb3IiLCJzdWIiOjI0LCJyb2xlIjoic3VwZXJfYWRtaW4iLCJ1dWlkIjoiM2I5ZjJiNTQtZTQzYS00MTBmLTkzZGMtYzQxY2MwYTI5MzhiIiwiY3VzdG9tZXJfdXVpZCI6bnVsbCwiY2xpZW50X3V1aWQiOm51bGwsImlhdCI6MTcxMTc0NTUyMywiZXhwIjoxNzExODMxOTIzfQ.KLHCO8IU_koIuRT2_7yaFOPVsL-rDDQwsxrHidi5d6GIDa232aT40LDt_1Xfjyft</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;store\&quot;: {\n        \&quot;description\&quot;: \&quot;WinInvestments Store\&quot;,\n        \&quot;external_reference\&quot;: \&quot;ni idea\&quot;,\n        \&quot;address\&quot;: \&quot;Pablo Nogues 654\&quot;,\n        \&quot;phone\&quot;: \&quot;1152472234\&quot;,\n        \&quot;manager\&quot;: \&quot;San Flow\&quot;\n        \n    }\n \n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>af100045-f29a-46dc-8d51-d11a8d279bc3</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzM4NCIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImFkbWluaXN0cmFkb3IiLCJzdWIiOjI0LCJyb2xlIjoic3VwZXJfYWRtaW4iLCJ1dWlkIjoiM2I5ZjJiNTQtZTQzYS00MTBmLTkzZGMtYzQxY2MwYTI5MzhiIiwiY3VzdG9tZXJfdXVpZCI6bnVsbCwiY2xpZW50X3V1aWQiOm51bGwsImlhdCI6MTcxMTc0NTUyMywiZXhwIjoxNzExODMxOTIzfQ.KLHCO8IU_koIuRT2_7yaFOPVsL-rDDQwsxrHidi5d6GIDa232aT40LDt_1Xfjyft</value>
      <webElementGuid>62d57136-7bc5-42bb-942b-fc2138918e71</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url_base}/store?collectorId=a37b0674-b4a4-45e9-b609-6321df5dc92c</restUrl>
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
      <id>d2752944-836a-4b64-8dc1-d4b7ced530de</id>
      <masked>false</masked>
      <name>url_base</name>
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
