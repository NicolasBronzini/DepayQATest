<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>3) Store - Crear sin POS (ADMIN)</name>
   <tag></tag>
   <elementGuidId>ab6cba21-a5f1-4e3d-8981-7a9799b146ff</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;store\&quot;: {\n        \&quot;description\&quot;: \&quot;Test Ecommerce Pablo Nogues\&quot;,\n        \&quot;external_reference\&quot;: \&quot;ni idea\&quot;,\n        \&quot;address\&quot;: \&quot;Pablo Nogues 654\&quot;,\n        \&quot;phone\&quot;: \&quot;1152472234\&quot;,\n        \&quot;manager\&quot;: \&quot;San Flow\&quot;\n    }\n   //\&quot;customer_uuid\&quot;: \&quot;19a9bdc4-95d0-4feb-a49b-b1cb8a44b94b\&quot; //si es un usuario ADMIN necesitas enviarle el customer_uuid si o si\n    //POR EJ SI HASAR QUIERE CREAR UNA STORE PARA UN CUSTOMER, DEBE PROPORCIONAR EL CUSTOMER_UUID\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url_base}/store/create?collectorId=6c1dc181-dcd2-45f0-9201-8204fb5908f9</restUrl>
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
      <id>de0398b1-488e-498e-9434-22c4e5b4452e</id>
      <masked>false</masked>
      <name>url_base</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
