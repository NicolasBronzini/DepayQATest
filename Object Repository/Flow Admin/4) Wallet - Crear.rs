<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>4) Wallet - Crear</name>
   <tag></tag>
   <elementGuidId>2437da46-0f75-4b62-9b6a-9002c20d490b</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;wallet\&quot;: {\n        \&quot;address\&quot;: \&quot;0xadresssadadasxxassasadadadaeedda\&quot;,\n        \&quot;network\&quot;: \&quot;ETH\&quot;,\n        \&quot;coin\&quot;: \&quot;USDT\&quot;\n    },\n    \&quot;store_uuid\&quot;: [\n        \&quot;3e18942e-2a14-4fae-9032-4c729b2d9ed1\&quot;\n    ]\n    //\&quot;customer_uuid\&quot;: \&quot;6f399208-1a8f-4781-bcfd-49d3f442d589\&quot; //customer cuando es para ecommerce, ya que no tienen branch\n\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url_base}/wallets/create?collectorId=c94dc96d-c391-41df-8a7a-aa5fcdc4d44e</restUrl>
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
      <id>9ef06a46-23fc-4eb5-a33e-bb964d3fb0f8</id>
      <masked>false</masked>
      <name>url_base</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
