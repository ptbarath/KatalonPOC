<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Update Contact</name>
   <tag></tag>
   <elementGuidId>279cd57a-cac1-4acb-8b26-1d30d3dca2ae</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot; {\n        \&quot;firstName\&quot;: \&quot;Chris\&quot;,\n        \&quot;lastName\&quot;: \&quot;John\&quot;,\n        \&quot;email\&quot;: \&quot;chrisjohn@gamil.com\&quot;,\n        \&quot;location\&quot;: {\n            \&quot;city\&quot;: \&quot;Bangalore\&quot;,\n            \&quot;country\&quot;: \&quot;India\&quot;\n        },\n        \&quot;employer\&quot;: {\n            \&quot;jobTitle\&quot;: \&quot;QA Lead\&quot;,\n            \&quot;company\&quot;: \&quot;${Company_Name}\&quot;\n        }\n }&quot;,
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
      <webElementGuid>d34d390a-7056-4610-b17b-c025ff1c00dd</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>http://3.13.86.142:3000/contacts/${ID}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.ID</defaultValue>
      <description></description>
      <id>ee9cff3d-ba00-478d-bc24-4d9092ace4cc</id>
      <masked>false</masked>
      <name>ID</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Company_Name</defaultValue>
      <description></description>
      <id>c6bae4e9-07df-4eed-9322-60a04c1f5e83</id>
      <masked>false</masked>
      <name>Company_Name</name>
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


WS.verifyResponseStatusCode(response, 204)

assertThat(response.getStatusCode()).isEqualTo(204)





ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
