<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>8) Exchange - Get (escanear QR) POS</name>
   <tag></tag>
   <elementGuidId>e4ff520a-41d7-441a-9d6c-63a4caf59680</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;qrValue\&quot;:\&quot;Depay:5a7d665c-ee2c-401e-a710-a0dc3fc34634/93ccd953-e378-49b6-a1e5-c289e7a5cd74/false\&quot;\n\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${url_prod}/exchange/GetPriceUstForOrderTotal</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url_prod</defaultValue>
      <description></description>
      <id>21cfc6e7-8291-47fb-9674-33f9d2620ef8</id>
      <masked>false</masked>
      <name>url_prod</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
