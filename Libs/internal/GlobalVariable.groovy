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
     * <p>Profile default : Uuid de la orden</p>
     */
    public static Object orderId
     
    /**
     * <p>Profile default : User tipo user POS</p>
     */
    public static Object api_key_pos
     
    /**
     * <p>Profile default : Depay token para hacer peticiones desde un admin para un user</p>
     */
    public static Object Depay_Token_User
     
    /**
     * <p>Profile default : store uuid del customer</p>
     */
    public static Object store_uuid
     
    /**
     * <p>Profile default : token tipo user</p>
     */
    public static Object token_user
     
    /**
     * <p>Profile default :  uuid del point of sales</p>
     */
    public static Object pos_uuid
     
    /**
     * <p>Profile default : external reference customer</p>
     */
    public static Object external_Customer
     
    /**
     * <p>Profile default : external reference point of sales</p>
     */
    public static Object external_pos
     
    /**
     * <p>Profile default : orderData de lectura de exchange</p>
     */
    public static Object orderData
     
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
            client_uuid = selectedVariables['client_uuid']
            api_key_ecommerce = selectedVariables['api_key_ecommerce']
            token_Admin = selectedVariables['token_Admin']
            orderId = selectedVariables['orderId']
            api_key_pos = selectedVariables['api_key_pos']
            Depay_Token_User = selectedVariables['Depay_Token_User']
            store_uuid = selectedVariables['store_uuid']
            token_user = selectedVariables['token_user']
            pos_uuid = selectedVariables['pos_uuid']
            external_Customer = selectedVariables['external_Customer']
            external_pos = selectedVariables['external_pos']
            orderData = selectedVariables['orderData']
            current_user = selectedVariables['current_user']
            
        } catch (Exception e) {
            TestCaseMain.logGlobalVariableError(e)
        }
    }
}
