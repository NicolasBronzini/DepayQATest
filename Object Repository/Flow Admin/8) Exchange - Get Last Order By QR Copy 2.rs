<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>8) Exchange - Get Last Order By QR Copy 2</name>
   <tag></tag>
   <elementGuidId>d0c26832-f2f3-4dda-ad86-d6b42ed4ae7a</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;qrValue\&quot;:\&quot;Depay:390b3098-ee3e-4c03-91ba-2c7c92f8e9cb/166c4f35-262b-4f4f-b752-ea2ebad84e02/false\&quot;\n\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
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
      <id>17e7519e-8159-4916-ab7a-f39d5a3dff3c</id>
      <masked>false</masked>
      <name>url_prod</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
