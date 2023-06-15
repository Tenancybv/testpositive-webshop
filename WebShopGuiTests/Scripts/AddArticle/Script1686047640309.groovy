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

//Set variables
int number1 = Math.floor(Math.random() * 9) + 2

int number2 = Math.floor(Math.random() * 9) + 2

String RN1 = number1.toString()

String RN2 = number2.toString()

int timeout = 90

int timeout2 = 15

int valueItem1 = 0

int valueItem2 = 0

WebUI.openBrowser('')

WebUI.navigateToUrl('https://demowebshop.tricentis.com/')

//Login
WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop/a_Log in'), timeout)

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop/a_Log in'))

WebUI.setText(findTestObject('Object Repository/Page_Demo Web Shop. Login/input_Email_Email'), 'GuiTest@gmail.com')

WebUI.setEncryptedText(findTestObject('Object Repository/Page_Demo Web Shop. Login/input_Password_Password'), 'rdcf2pvC1eAK0Pf1WCwicA==')

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Login/input_Forgot password_button-1 login-button'))

//remove any existing items from cart
WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop/span_Shopping cart'), timeout)

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop/span_Shopping cart'))

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Remove_removefromcart'), 
    timeout2,  FailureHandling.OPTIONAL) == true) {
    WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Remove_removefromcart'))

    WebUI.click(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Total_updatecart'), FailureHandling.OPTIONAL)
} else {
}

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop/span_Shopping cart'), FailureHandling.OPTIONAL)

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Remove_removefromcart'), 
    timeout2,  FailureHandling.OPTIONAL) == true) {
    WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Remove_removefromcart'))

    WebUI.click(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Total_updatecart'), FailureHandling.OPTIONAL)
} else {
}

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop/span_Shopping cart'), FailureHandling.OPTIONAL)

if (WebUI.verifyElementPresent(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Remove_removefromcart'), 
    timeout2,  FailureHandling.OPTIONAL) == true) {
    WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Remove_removefromcart'))

    WebUI.click(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/input_Total_updatecart'), FailureHandling.OPTIONAL)
} else {
}

//navigate to cell phones
WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop/a_Electronics'), timeout)

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop/a_Electronics'))

WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop. Electronics/a_Cell phones'), timeout)

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Electronics/a_Cell phones'))

Random rand = new Random()

upperLimit = 2

icnt = rand.nextInt(upperLimit)

switch (icnt) {
    case 0:
        WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop. Cell phones/a_Smartphone'), 
            timeout)

        WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Cell phones/a_Smartphone'))

        WebUI.setText(findTestObject('Object Repository/Page_Demo Web Shop. Smartphone/input_Qty_addtocart_43.EnteredQuantity'), 
            RN1)

        WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Smartphone/input_Qty_add-to-cart-button-43'))

        valueItem1 = (number1 * 100)

        break
    case 1:
        WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop. Cell phones/a_Used phone'), 
            timeout)

        WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Cell phones/a_Used phone'))

        WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Used phone/div_Add your review_add-to-cart'))

        break
    default:
        WebUI.comment('Whoops')

        break
}

//navigate to apparal
WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop. Blue Jeans/a_Apparel  Shoes'), timeout)

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Blue Jeans/a_Apparel  Shoes'))

Random rand2 = new Random()

upperLimit2 = 2

icnt2 = rand2.nextInt(upperLimit2)

switch (icnt2) {
    case 0:
        WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop. Apparel  Shoes/img'), timeout)

        WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Apparel  Shoes/a_Blue Jeans'))

        WebUI.setText(findTestObject('Object Repository/Page_Demo Web Shop. Blue Jeans/input_Qty_addtocart_36.EnteredQuantity'), 
            RN2)

        WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Blue Jeans/input_Qty_add-to-cart-button-36'))

        valueItem2 = (number2 * 1)

        break
    case 1:
        WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop. Apparel  Shoes/a_Casual Golf Belt'), 
            timeout)

        WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Apparel  Shoes/a_Casual Golf Belt'))

        WebUI.setText(findTestObject('Object Repository/Page_Demo Web Shop. Casual Golf Belt/input_Qty_addtocart_40.EnteredQuantity'), 
            RN2)

        WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Casual Golf Belt/input_Qty_add-to-cart-button-40'))

        valueItem2 = (number2 * 1)

        break
    default:
        WebUI.comment('Whoops')

        break
}

//check cart and log out'
int totalprice = valueItem1 + valueItem2

WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop. Blue Jeans/span_Shopping cart'), timeout)

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Blue Jeans/span_Shopping cart'))

WebUI.verifyElementText(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/Sub-total'), totalprice.toString() + 
    '.00')

WebUI.waitForElementClickable(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/a_Log out'), timeout)

WebUI.enhancedClick(findTestObject('Object Repository/Page_Demo Web Shop. Shopping Cart/a_Log out'))

WebUI.closeBrowser()
