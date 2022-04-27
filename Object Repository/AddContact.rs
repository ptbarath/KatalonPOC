<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>AddContact</name>
   <tag></tag>
   <elementGuidId>ed9329e2-02cb-4720-bd35-23431069c0e1</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot; {\n        \&quot;firstName\&quot;: \&quot;Chris\&quot;,\n        \&quot;lastName\&quot;: \&quot;John\&quot;,\n        \&quot;email\&quot;: \&quot;chrisjohn@gamil.com\&quot;,\n        \&quot;location\&quot;: {\n            \&quot;city\&quot;: \&quot;Bangalore\&quot;,\n            \&quot;country\&quot;: \&quot;India\&quot;\n        },\n        \&quot;employer\&quot;: {\n            \&quot;jobTitle\&quot;: \&quot;QA Lead\&quot;,\n            \&quot;company\&quot;: \&quot;CTS-USA\&quot;\n        }\n }&quot;,
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
      <webElementGuid>db3716f6-4777-4010-8e67-9fde03a2527d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.2.5</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://3.13.86.142:3000/contacts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>d6545904-7259-4a2c-a1b4-daf2d9965c34</id>
      <masked>false</masked>
      <name>ID</name>
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


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)


assertThat(response.getResponseText()).contains('Katalon Test Project')
WS.verifyElementPropertyValue(response, '_id', &quot;6267931bf2967f0ec89589e4&quot;)



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
