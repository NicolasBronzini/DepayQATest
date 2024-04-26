<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Get Order</name>
   <tag></tag>
   <elementGuidId>5dbcd955-f4a2-4ec2-9f99-a212ce8cc91d</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzM4NCIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImFkbWluaXN0cmFkb3IiLCJzdWIiOjI0LCJyb2xlIjoic3VwZXJfYWRtaW4iLCJ1dWlkIjoiM2I5ZjJiNTQtZTQzYS00MTBmLTkzZGMtYzQxY2MwYTI5MzhiIiwiY3VzdG9tZXJfdXVpZCI6bnVsbCwiY2xpZW50X3V1aWQiOm51bGwsImlhdCI6MTcxMjY3NTk3OCwiZXhwIjoxNzEyNzYyMzc4fQ.d4-w_vggWy23QvcVkmMKOlr20cEYvRaaY0QWZWZxTryYM0M-93i6KlbuSmPp-xqZ</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzM4NCIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImFkbWluaXN0cmFkb3IiLCJzdWIiOjI0LCJyb2xlIjoic3VwZXJfYWRtaW4iLCJ1dWlkIjoiM2I5ZjJiNTQtZTQzYS00MTBmLTkzZGMtYzQxY2MwYTI5MzhiIiwiY3VzdG9tZXJfdXVpZCI6bnVsbCwiY2xpZW50X3V1aWQiOm51bGwsImlhdCI6MTcxMjY3NTk3OCwiZXhwIjoxNzEyNzYyMzc4fQ.d4-w_vggWy23QvcVkmMKOlr20cEYvRaaY0QWZWZxTryYM0M-93i6KlbuSmPp-xqZ</value>
      <webElementGuid>130cc037-0c90-408f-8fa7-813d2b16b365</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://test.depayapp.com/order/ecommerce/0b06eede-aa48-49ca-b4b1-14c64ecfcc16?origin=Belo</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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
