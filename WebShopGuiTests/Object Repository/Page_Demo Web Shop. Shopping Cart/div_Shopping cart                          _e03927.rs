<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Shopping cart                          _e03927</name>
   <tag></tag>
   <elementGuidId>234f8ea2-599e-47ea-bcb0-af034cd35ff8</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>div.page.shopping-cart-page</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>(.//*[normalize-space(text()) and normalize-space(.)='Gift Cards'])[2]/following::div[6]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
      <webElementGuid>2d5d7f93-3d6c-4c30-8850-f4c28cbec630</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>page shopping-cart-page</value>
      <webElementGuid>b06a2f1c-8305-4450-9a15-29ed1a71f2ad</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            1.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
    
        
            $(function () {
                $(&quot;#CountryId&quot;).change(function () {
                    var selectedItem = $(this).val();
                    var ddlStates = $(&quot;#StateProvinceId&quot;);
                    var estimateProgress = $(&quot;#estimate-shipping-loading-progress&quot;);
                    estimateProgress.show();
                    $.ajax({
                        cache: false,
                        type: &quot;GET&quot;,
                        url: &quot;/country/getstatesbycountryid&quot;,
                 data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                 success: function (data) {
                     ddlStates.html('');
                     $.each(data, function (id, option) {
                         ddlStates.append($('&lt;option>&lt;/option>').val(option.id).html(option.name));
                     });
                     estimateProgress.hide();
                 },
                 error: function (xhr, ajaxOptions, thrownError) {
                     alert('Failed to retrieve states.');
                     estimateProgress.hide();
                 }
             });
                });
            });
        

        
            
                Estimate shipping
            
            Enter your destination to get a shipping estimate
            
                
                    Country:
                    Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D'Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People's Republic of
Kuwait
Kyrgyzstan
Lao People's Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

                    *
                
                
                    State / province:
                    Other (Non US)

                    Wait...
                
                
                    Zip / postal code:
                    
                
                
                    
                
            
        
    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    1.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Calculated during checkout
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            Calculated during checkout
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $('#checkout').click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($('#termsofservice').length > 0) {
                                            //terms of service element exists
                                            if (!$('#termsofservice').is(':checked')) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    
</value>
      <webElementGuid>6a901a49-70e4-49ae-90fd-474b36c65304</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>/html[1]/body[1]/div[@class=&quot;master-wrapper-page&quot;]/div[@class=&quot;master-wrapper-content&quot;]/div[@class=&quot;master-wrapper-main&quot;]/div[@class=&quot;center-1&quot;]/div[@class=&quot;page shopping-cart-page&quot;]</value>
      <webElementGuid>bd761d4a-f4c3-4fcd-8cd9-b3665f816a85</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Gift Cards'])[2]/following::div[6]</value>
      <webElementGuid>1df34718-ce6c-4539-807b-cb56cb3cc163</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Jewelry'])[2]/following::div[7]</value>
      <webElementGuid>256817d7-6be3-419a-bc55-9ba3977bfca7</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div/div[4]/div/div</value>
      <webElementGuid>716e4f2e-0aa9-4cdc-8438-dc983fd77dd9</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//div[(text() = concat(&quot;
    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            1.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
    
        
            $(function () {
                $(&quot;#CountryId&quot;).change(function () {
                    var selectedItem = $(this).val();
                    var ddlStates = $(&quot;#StateProvinceId&quot;);
                    var estimateProgress = $(&quot;#estimate-shipping-loading-progress&quot;);
                    estimateProgress.show();
                    $.ajax({
                        cache: false,
                        type: &quot;GET&quot;,
                        url: &quot;/country/getstatesbycountryid&quot;,
                 data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                 success: function (data) {
                     ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                     $.each(data, function (id, option) {
                         ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                     });
                     estimateProgress.hide();
                 },
                 error: function (xhr, ajaxOptions, thrownError) {
                     alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                     estimateProgress.hide();
                 }
             });
                });
            });
        

        
            
                Estimate shipping
            
            Enter your destination to get a shipping estimate
            
                
                    Country:
                    Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

                    *
                
                
                    State / province:
                    Other (Non US)

                    Wait...
                
                
                    Zip / postal code:
                    
                
                
                    
                
            
        
    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    1.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Calculated during checkout
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            Calculated during checkout
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $(&quot; , &quot;'&quot; , &quot;#checkout&quot; , &quot;'&quot; , &quot;).click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).length > 0) {
                                            //terms of service element exists
                                            if (!$(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    
&quot;) or . = concat(&quot;
    
        Shopping cart
    
    
        

    
    
        
            
                    
                                                    
                
                
                
                
            
            
                
                        
                            Remove
                        
                                                                
                    
                        Product(s)
                    
                    
                        Price
                    
                    
                        Qty.
                    
                    
                        Total
                    
                
            
            
                    
                            
                                Remove:
                                
                            
                                                                            
                                
                            
                        
                            Blue Jeans
                                                                                                            
                        
                            Price:
                            1.00
                        
                        
                            Qty.:
                                    
                        
                        
                            Total:
                            1.00
                        
                    
            
        
        
                
                    
                    
                
        
        
            
            
                    
                            
        
            Discount Code
        
        
            Enter your coupon here
        
        
            
            
        
            

                            
        
            Gift Cards
        
        Enter gift card code
        
            
            
        
    

                        
                    
    
        
            $(function () {
                $(&quot;#CountryId&quot;).change(function () {
                    var selectedItem = $(this).val();
                    var ddlStates = $(&quot;#StateProvinceId&quot;);
                    var estimateProgress = $(&quot;#estimate-shipping-loading-progress&quot;);
                    estimateProgress.show();
                    $.ajax({
                        cache: false,
                        type: &quot;GET&quot;,
                        url: &quot;/country/getstatesbycountryid&quot;,
                 data: { &quot;countryId&quot;: selectedItem, &quot;addEmptyStateIfRequired&quot;: &quot;true&quot; },
                 success: function (data) {
                     ddlStates.html(&quot; , &quot;'&quot; , &quot;&quot; , &quot;'&quot; , &quot;);
                     $.each(data, function (id, option) {
                         ddlStates.append($(&quot; , &quot;'&quot; , &quot;&lt;option>&lt;/option>&quot; , &quot;'&quot; , &quot;).val(option.id).html(option.name));
                     });
                     estimateProgress.hide();
                 },
                 error: function (xhr, ajaxOptions, thrownError) {
                     alert(&quot; , &quot;'&quot; , &quot;Failed to retrieve states.&quot; , &quot;'&quot; , &quot;);
                     estimateProgress.hide();
                 }
             });
                });
            });
        

        
            
                Estimate shipping
            
            Enter your destination to get a shipping estimate
            
                
                    Country:
                    Select country
United States
Canada
Afghanistan
Albania
Algeria
American Samoa
Andorra
Angola
Anguilla
Antarctica
Antigua and Barbuda
Argentina
Armenia
Aruba
Australia
Austria
Azerbaijan
Bahamas
Bahrain
Bangladesh
Barbados
Belarus
Belgium
Belize
Benin
Bermuda
Bhutan
Bolivia
Bosnia and Herzegowina
Botswana
Bouvet Island
Brazil
British Indian Ocean Territory
Brunei Darussalam
Bulgaria
Burkina Faso
Burundi
Cambodia
Cameroon
Cape Verde
Cayman Islands
Central African Republic
Chad
Chile
China
Christmas Island
Cocos (Keeling) Islands
Colombia
Comoros
Congo
Cook Islands
Costa Rica
Cote D&quot; , &quot;'&quot; , &quot;Ivoire
Croatia
Cuba
Cyprus
Czech Republic
Denmark
Djibouti
Dominica
Dominican Republic
Ecuador
Egypt
El Salvador
Equatorial Guinea
Eritrea
Estonia
Ethiopia
Falkland Islands (Malvinas)
Faroe Islands
Fiji
Finland
France
French Guiana
French Polynesia
French Southern Territories
Gabon
Gambia
Georgia
Germany
Ghana
Gibraltar
Greece
Greenland
Grenada
Guadeloupe
Guam
Guatemala
Guinea
Guinea-bissau
Guyana
Haiti
Heard and Mc Donald Islands
Honduras
Hong Kong
Hungary
Iceland
India
Indonesia
Iran (Islamic Republic of)
Iraq
Ireland
Israel
Italy
Jamaica
Japan
Jordan
Kazakhstan
Kenya
Kiribati
Korea
Korea, Democratic People&quot; , &quot;'&quot; , &quot;s Republic of
Kuwait
Kyrgyzstan
Lao People&quot; , &quot;'&quot; , &quot;s Democratic Republic
Latvia
Lebanon
Lesotho
Liberia
Libyan Arab Jamahiriya
Liechtenstein
Lithuania
Luxembourg
Macau
Macedonia
Madagascar
Malawi
Malaysia
Maldives
Mali
Malta
Marshall Islands
Martinique
Mauritania
Mauritius
Mayotte
Mexico
Micronesia
Moldova
Monaco
Mongolia
Montenegro
Montserrat
Morocco
Mozambique
Myanmar
Namibia
Nauru
Nepal
Netherlands
Netherlands Antilles
New Caledonia
New Zealand
Nicaragua
Niger
Nigeria
Niue
Norfolk Island
Northern Mariana Islands
Norway
Oman
Pakistan
Palau
Panama
Papua New Guinea
Paraguay
Peru
Philippines
Pitcairn
Poland
Portugal
Puerto Rico
Qatar
Reunion
Romania
Russia
Rwanda
Saint Kitts and Nevis
Saint Lucia
Saint Vincent and the Grenadines
Samoa
San Marino
Sao Tome and Principe
Saudi Arabia
Senegal
Serbia
Seychelles
Sierra Leone
Singapore
Slovakia (Slovak Republic)
Slovenia
Solomon Islands
Somalia
South Africa
South Georgia &amp; South Sandwich Islands
Spain
Sri Lanka
St. Helena
St. Pierre and Miquelon
Sudan
Suriname
Svalbard and Jan Mayen Islands
Swaziland
Sweden
Switzerland
Syrian Arab Republic
Taiwan
Tajikistan
Tanzania
Thailand
Togo
Tokelau
Tonga
Trinidad and Tobago
Tunisia
Turkey
Turkmenistan
Turks and Caicos Islands
Tuvalu
Uganda
Ukraine
United Arab Emirates
United Kingdom
United States minor outlying islands
Uruguay
Uzbekistan
Vanuatu
Vatican City State (Holy See)
Venezuela
Viet Nam
Virgin Islands (British)
Virgin Islands (U.S.)
Wallis and Futuna Islands
Western Sahara
Yemen
Zambia
Zimbabwe

                    *
                
                
                    State / province:
                    Other (Non US)

                    Wait...
                
                
                    Zip / postal code:
                    
                
                
                    
                
            
        
    
            
            
                
    
        
            
                
                    Sub-Total:
                
                
                    1.00 
                
            
            
                
                    
                        Shipping:
                
                
                    
                            Calculated during checkout
                    
                
            
                                        
                    
                        Tax: 
                    
                    
                        0.00 
                    
                
                                                
                
                    
                        Total:
                
                
                    
                            Calculated during checkout
                    
                
            
        
    


                        
                            Please accept the terms of service before the next step.
                        
                        
                            
                            I agree with the terms of service and I adhere to them unconditionally
                            (read)
                        
                    
                            
                                $(document).ready(function () {
                                    $(&quot; , &quot;'&quot; , &quot;#checkout&quot; , &quot;'&quot; , &quot;).click(function () {
                                        //terms of service
                                        var termOfServiceOk = true;
                                        if ($(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).length > 0) {
                                            //terms of service element exists
                                            if (!$(&quot; , &quot;'&quot; , &quot;#termsofservice&quot; , &quot;'&quot; , &quot;).is(&quot; , &quot;'&quot; , &quot;:checked&quot; , &quot;'&quot; , &quot;)) {
                                                $(&quot;#terms-of-service-warning-box&quot;).dialog();
                                                termOfServiceOk = false;
                                            } else {
                                                termOfServiceOk = true;
                                            }
                                        }
                                        return termOfServiceOk;
                                    });
                                });
                            
                            
                                Checkout
                            
                    
                    
                        
                        
                    
            
        
    


    
&quot;))]</value>
      <webElementGuid>d57c6a52-b375-49c6-a38d-506cdb9be7b4</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
