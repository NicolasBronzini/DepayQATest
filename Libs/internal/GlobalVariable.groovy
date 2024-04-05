package internal

import com.kms.katalon.core.configuration.RunConfiguration
import com.kms.katalon.core.main.TestCaseMain


/**
 * This class is generated automatically by Katalon Studio and should not be modified or deleted.
 */
public class GlobalVariable {
     
    /**
     * <p>Profile default : URL de test
Profile Depay : Url de la api de test
Profile Hasar : Api de test</p>
     */
    public static Object url_base
     
    /**
     * <p>Profile default : DEPAY SUPER
Profile Depay : User Super Admin
Profile Hasar : Tipo Admin</p>
     */
    public static Object api_key
     
    /**
     * <p>Profile default : HASAR</p>
     */
    public static Object api_key_hasar
     
    /**
     * <p>Profile Hasar : Bearer_token</p>
     */
    public static Object token
     
    /**
     * <p>Profile default : uuid del customer</p>
     */
    public static Object customer_uuid
     
    /**
     * <p>Profile default : Api_key del usuario creado</p>
     */
    public static Object api_key_user
     
    /**
     * <p>Profile default : Token del user tipo user</p>
     */
    public static Object access_token
     
    /**
     * <p>Profile default : UUID cliente ecomercce</p>
     */
    public static Object client_uuid
     
    /**
     * <p>Profile default : User tipo admin ecommerce</p>
     */
    public static Object api_key_ecommerce
     
    /**
     * <p>Profile default : token_Admin</p>
     */
    public static Object token_Admin
     
    /**
     * <p>Profile Hasar : Distentivo para testear user</p>
     */
    public static Object current_user
     

    static {
        try {
            def selectedVariables = TestCaseMain.getGlobalVariables("default")
			selectedVariables += TestCaseMain.getGlobalVariables(RunConfiguration.getExecutionProfile())
            selectedVariables += TestCaseMain.getParsedValues(RunConfiguration.getOverridingParameters(), selectedVariables)
    
            url_base = selectedVariables['url_base']
            api_key = selectedVariables['api_key']
            api_key_hasar = selectedVariables['api_key_hasar']
            token = selectedVariables['token']
            customer_uuid = selectedVariables['customer_uuid']
            api_key_user = selectedVariables['api_key_user']
            access_token = selectedVariables['access_token']
            client_uuid = selectedVariables['client_uuid']
            api_key_ecommerce = selectedVariables['api_key_ecommerce']
            token_Admin = selectedVariables['token_Admin']
            current_user = selectedVariables['current_user']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
