import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

response = WS.sendRequest(findTestObject('USERS/Post Users', [('name') : name, ('username') : username, ('email') : email
            , ('street') : street, ('suite') : suite, ('city') : city, ('zipcode') : zipcode, ('lat') : lat, ('lng') : lng
            , ('phone') : phone, ('website') : website, ('companyName') : companyName, ('catchPhrase') : catchPhrase, ('bs') : bs]))

WS.verifyResponseStatusCode(response, 201)

WS.verifyElementPropertyValue(response, 'name', name)

WS.verifyElementPropertyValue(response, 'username', username)

WS.verifyElementPropertyValue(response, 'address.street', street)

WS.verifyElementPropertyValue(response, 'address.suite', suite)

WS.verifyElementPropertyValue(response, 'address.city', city)

WS.verifyElementPropertyValue(response, 'address.zipcode', zipcode)

WS.verifyElementPropertyValue(response, 'address.geo.lat', lat)

WS.verifyElementPropertyValue(response, 'address.geo.lng', lng)

WS.verifyElementPropertyValue(response, 'phone', phone)

WS.verifyElementPropertyValue(response, 'website', website)

WS.verifyElementPropertyValue(response, 'company.name', companyName)

WS.verifyElementPropertyValue(response, 'company.catchPhrase', catchPhrase)

WS.verifyElementPropertyValue(response, 'company.bs', bs)

