<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>7.1) Order - Crear una con sus items (ECOMMERCE)</name>
   <tag></tag>
   <elementGuidId>8e9e8973-c8f2-416c-8e09-80d74791c615</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;Origin\&quot;: \&quot;e-commerce\&quot;,\n   \&quot;Order\&quot;: {\n        \&quot;external_reference\&quot;: \&quot;EXT_REF_9797\&quot;,\n        \&quot;total_amount\&quot;: 0.01,\n        \&quot;description\&quot;: \&quot;belo description\&quot;,\n        \&quot;title\&quot;: \&quot;testing order 444\&quot;,\n        \&quot;notification_url\&quot;: \&quot;https://zqmh0a9jg3.execute-api.sa-east-1.amazonaws.com/Prod/listenorder\&quot;\n    },\n    \&quot;items\&quot;: [\n        {\n            \&quot;sku_number\&quot;: \&quot;serie\&quot;,\n            \&quot;category\&quot;: \&quot;category\&quot;,\n            \&quot;title\&quot;: \&quot;title\&quot;,\n            \&quot;description\&quot;: \&quot;description\&quot;,\n            \&quot;quantity\&quot;: 3,\n            \&quot;unit_measure\&quot;: \&quot;unit\&quot;,\n            \&quot;unit_price\&quot;: \&quot;500\&quot;,\n            \&quot;total_amount\&quot;: 1,\n            \&quot;currency_id\&quot;: \&quot;USDT\&quot;\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${url_local}/order/create/ecommerce?collectorId=fa2314b4-31db-487e-bc6e-60daad0e6852</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.url_local</defaultValue>
      <description></description>
      <id>f66c496b-18ab-4fda-ab8a-4d62ccac4ef4</id>
      <masked>false</masked>
      <name>url_local</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
