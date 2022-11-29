<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Post Users</name>
   <tag></tag>
   <elementGuidId>41c90a2a-74d1-470b-80b6-54cc72c384bf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;name\&quot;: \&quot;${name}\&quot;,\n    \&quot;username\&quot;: \&quot;${username}\&quot;,\n    \&quot;email\&quot;: \&quot;${email}\&quot;,\n    \&quot;address\&quot;: {\n      \&quot;street\&quot;: \&quot;${street}\&quot;,\n      \&quot;suite\&quot;: \&quot;${suite}\&quot;,\n      \&quot;city\&quot;: \&quot;${city}\&quot;,\n      \&quot;zipcode\&quot;: \&quot;${zipcode}\&quot;,\n      \&quot;geo\&quot;: {\n        \&quot;lat\&quot;: \&quot;${lat}\&quot;,\n        \&quot;lng\&quot;: \&quot;${lng}\&quot;\n      }\n    },\n    \&quot;phone\&quot;: \&quot;${phone}\&quot;,\n    \&quot;website\&quot;: \&quot;${website}\&quot;,\n    \&quot;company\&quot;: {\n      \&quot;name\&quot;: \&quot;${companyName}\&quot;,\n      \&quot;catchPhrase\&quot;: \&quot;${catchPhrase}\&quot;,\n      \&quot;bs\&quot;: \&quot;${bs}\&quot;\n    }\n  }&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>8.0.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BaseURL}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Neymar Jr'</defaultValue>
      <description></description>
      <id>d01dd10a-2914-466b-a1fc-32c603d7208a</id>
      <masked>false</masked>
      <name>name</name>
   </variables>
   <variables>
      <defaultValue>'Neymar'</defaultValue>
      <description></description>
      <id>fb10dcc0-4650-4180-9d1d-d4efdf35184e</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'neymar@brazil.co.id'</defaultValue>
      <description></description>
      <id>2c5e762a-9ba2-4f9b-b35b-1e380b244888</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'Ganda Street'</defaultValue>
      <description></description>
      <id>d41bcfff-e73d-41ca-91c3-fccdeb5bbda5</id>
      <masked>false</masked>
      <name>street</name>
   </variables>
   <variables>
      <defaultValue>'Apt. 100'</defaultValue>
      <description></description>
      <id>457dc747-c8cc-4459-aa8d-8d8843f560a2</id>
      <masked>false</masked>
      <name>suite</name>
   </variables>
   <variables>
      <defaultValue>'Mexico'</defaultValue>
      <description></description>
      <id>d429760b-e3c6-4e3d-b90a-72ad7f4d1963</id>
      <masked>false</masked>
      <name>city</name>
   </variables>
   <variables>
      <defaultValue>'89023'</defaultValue>
      <description></description>
      <id>5fe7ed6a-518a-48fd-af98-6f5e9e5f1311</id>
      <masked>false</masked>
      <name>zipcode</name>
   </variables>
   <variables>
      <defaultValue>'-37.3159'</defaultValue>
      <description></description>
      <id>6bd9329c-5e78-4c46-83f5-9d294c206c5b</id>
      <masked>false</masked>
      <name>lat</name>
   </variables>
   <variables>
      <defaultValue>'81.1496'</defaultValue>
      <description></description>
      <id>f1e00dcf-3250-48d7-8bfe-9c82d9f385f2</id>
      <masked>false</masked>
      <name>lng</name>
   </variables>
   <variables>
      <defaultValue>'7829-0927'</defaultValue>
      <description></description>
      <id>cda7bb24-4018-499a-aea0-ae390d73da42</id>
      <masked>false</masked>
      <name>phone</name>
   </variables>
   <variables>
      <defaultValue>'neymar.org'</defaultValue>
      <description></description>
      <id>714acaca-d867-400e-9a93-ecea329fe2cf</id>
      <masked>false</masked>
      <name>website</name>
   </variables>
   <variables>
      <defaultValue>'neymar corp'</defaultValue>
      <description></description>
      <id>1fcfdf89-5dc6-4957-ad7c-447c7ed5c8fd</id>
      <masked>false</masked>
      <name>companyName</name>
   </variables>
   <variables>
      <defaultValue>'football-uni'</defaultValue>
      <description></description>
      <id>5dc821fa-55d3-4d0d-be70-a265421ec58a</id>
      <masked>false</masked>
      <name>catchPhrase</name>
   </variables>
   <variables>
      <defaultValue>'go to football club'</defaultValue>
      <description></description>
      <id>97628a1e-150f-41ff-b2e3-e4e2b99627ca</id>
      <masked>false</masked>
      <name>bs</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
