<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>7) Order - Crear una con sus items</name>
   <tag></tag>
   <elementGuidId>a2735a64-dd0a-4c01-98e3-64f08969124c</elementGuidId>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;wallet\&quot;:\&quot;Belo\&quot;,\n    \&quot;network\&quot;:\&quot;ETH\&quot;,\n    \&quot;coin\&quot;:\&quot;ETH\&quot;,\n    \&quot;fiat\&quot;:\&quot;ARS\&quot;, //ARS,BRL,VEF,BOB BOV,UYU,CLF CLP,MXN MXV,COP  COU,PEN. Monedas que se pueden cotizar de LATAM\n    \&quot;external_reference\&quot;: \&quot;0000010000020000011500000000000249160910\&quot;,\n    \&quot;total_amount\&quot;: 1000,\n    \&quot;description\&quot;: \&quot;Compra en HASAR\&quot;,\n    \&quot;title\&quot;: \&quot;Compra en HASAR\&quot;,\n    \&quot;notification_url\&quot;: \&quot;https://typedwebhook.tools/webhook/fab28b4a-60d2-41e5-b3b5-e90bf9cfc903?source_news\u003dipn\&quot;,\n   \n    \&quot;items\&quot;: [\n        {\n            \&quot;sku_number\&quot;: \&quot;serie\&quot;,\n            \&quot;category\&quot;: \&quot;category\&quot;,\n            \&quot;title\&quot;: \&quot;title\&quot;,\n            \&quot;description\&quot;: \&quot;description\&quot;,\n            \&quot;quantity\&quot;: 3,\n            \&quot;unit_measure\&quot;: \&quot;unit\&quot;,\n            \&quot;unit_price\&quot;: \&quot;500\&quot;,\n            \&quot;total_amount\&quot;: 1500,\n            \&quot;currency_id\&quot;: \&quot;USDT\&quot;\n        },\n                {\n            \&quot;sku_number\&quot;: \&quot;serie 2\&quot;,\n            \&quot;category\&quot;: \&quot;category\&quot;,\n            \&quot;title\&quot;: \&quot;title\&quot;,\n            \&quot;description\&quot;: \&quot;description\&quot;,\n            \&quot;quantity\&quot;: 3,\n            \&quot;unit_measure\&quot;: \&quot;unit\&quot;,\n            \&quot;unit_price\&quot;: \&quot;500\&quot;,\n            \&quot;total_amount\&quot;: 1500,\n            \&quot;currency_id\&quot;: \&quot;USDT\&quot;\n        }\n    ]\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>x-api-key</name>
      <type>Main</type>
      <value>${api_key}</value>
      <webElementGuid>aa7a9664-e464-4ed1-a943-6adde64e9ddc</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.2</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${url_prod}/instore/qr/seller/collectors/614250064/pos/002001/orders?access_token=DEPAY_TOKEN-afd37d212d6f55491a1e60c90310d37e6593a0324457776153d137797f1a29e8</restUrl>
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
      <id>d975a267-3388-4ebc-b306-777c79857356</id>
      <masked>false</masked>
      <name>url_prod</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.api_key</defaultValue>
      <description></description>
      <id>b5a9bd82-9359-4491-aa99-fb8c76719a9f</id>
      <masked>false</masked>
      <name>api_key</name>
   </variables>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
