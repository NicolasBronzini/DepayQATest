<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>9) Exchange - Respuesta</name>
   <tag></tag>
   <elementGuidId>a4f214b5-9f34-4102-906f-51271f695e5e</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n \&quot;qrValue\&quot;:\&quot;Depay:5a7d665c-ee2c-401e-a710-a0dc3fc34634/93ccd953-e378-49b6-a1e5-c289e7a5cd74/false\&quot;, // customer/pos_uuid or order_uuid/is_attend\n  \&quot;transaction\&quot;: {\n    \&quot;wallet_origin\&quot;: \&quot;flo_wallet\&quot;,\n    \&quot;transaction_number\&quot;: \&quot;flo_111\&quot;,\n    \&quot;amount\&quot;:498008,\n    \&quot;status\&quot;: \&quot;closed\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url_base}/exchange/createHasar</restUrl>
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
      <id>b21d6360-ca4f-4625-8fe9-4843b4bb35d3</id>
      <masked>false</masked>
      <name>url_base</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
