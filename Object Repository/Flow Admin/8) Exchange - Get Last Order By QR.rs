<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>8) Exchange - Get Last Order By QR</name>
   <tag></tag>
   <elementGuidId>b160798f-da70-44f9-a4e1-541195c47b03</elementGuidId>
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
   <restUrl>${url_prod}/exchange/order/by-qr</restUrl>
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
      <id>07d6caef-52a1-42f2-b994-9f78f2c96de0</id>
      <masked>false</masked>
      <name>url_prod</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
