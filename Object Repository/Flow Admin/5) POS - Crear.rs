<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>5) POS - Crear</name>
   <tag></tag>
   <elementGuidId>da239777-678e-4cd3-8622-b64a5256473a</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;pos\&quot;: {\n        \&quot;description\&quot;: \&quot;TestFlowPos\&quot;,\n        \&quot;fixed_amount\&quot;: false,\n        \&quot;category\&quot;: 5,\n        \&quot;external_reference\&quot;: \&quot;test-flow-pos-1234567\&quot;\n    }\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url_base}/pointofsales/create/?store_uuid=3e18942e-2a14-4fae-9032-4c729b2d9ed1&amp;collectorId=6c1dc181-dcd2-45f0-9201-8204fb5908f9</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url_base</defaultValue>
      <description></description>
      <id>1a791c9f-e549-4248-ac66-f3f956b09512</id>
      <masked>false</masked>
      <name>url_base</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
