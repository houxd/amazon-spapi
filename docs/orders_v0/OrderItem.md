# OrderItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**asin** | **String** | The item's Amazon Standard Identification Number (ASIN). | 
**seller_sku** | Option<**String**> | The item's seller stock keeping unit (SKU). | [optional]
**order_item_id** | **String** | An Amazon-defined order item identifier. | 
**associated_items** | Option<[**Vec<models::AssociatedItem>**](AssociatedItem.md)> | A list of associated items that a customer has purchased with a product. For example, a tire installation service purchased with tires. | [optional]
**title** | Option<**String**> | The item's name. | [optional]
**quantity_ordered** | **i32** | The number of items in the order.  | 
**quantity_shipped** | Option<**i32**> | The number of items shipped. | [optional]
**product_info** | Option<[**models::ProductInfoDetail**](ProductInfoDetail.md)> |  | [optional]
**points_granted** | Option<[**models::PointsGrantedDetail**](PointsGrantedDetail.md)> |  | [optional]
**item_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**shipping_price** | Option<[**models::Money**](Money.md)> |  | [optional]
**item_tax** | Option<[**models::Money**](Money.md)> |  | [optional]
**shipping_tax** | Option<[**models::Money**](Money.md)> |  | [optional]
**shipping_discount** | Option<[**models::Money**](Money.md)> |  | [optional]
**shipping_discount_tax** | Option<[**models::Money**](Money.md)> |  | [optional]
**promotion_discount** | Option<[**models::Money**](Money.md)> |  | [optional]
**promotion_discount_tax** | Option<[**models::Money**](Money.md)> |  | [optional]
**promotion_ids** | Option<**Vec<String>**> | A list of promotion identifiers provided by the seller when the promotions were created. | [optional]
**cod_fee** | Option<[**models::Money**](Money.md)> |  | [optional]
**cod_fee_discount** | Option<[**models::Money**](Money.md)> |  | [optional]
**is_gift** | Option<**String**> | Indicates whether the item is a gift.  **Possible values**: `true` and `false`. | [optional]
**condition_note** | Option<**String**> | The condition of the item, as described by the seller. | [optional]
**condition_id** | Option<**String**> | The condition of the item.  **Possible values**: `New`, `Used`, `Collectible`, `Refurbished`, `Preorder`, and `Club`. | [optional]
**condition_subtype_id** | Option<**String**> | The subcondition of the item.  **Possible values**: `New`, `Mint`, `Very Good`, `Good`, `Acceptable`, `Poor`, `Club`, `OEM`, `Warranty`, `Refurbished Warranty`, `Refurbished`, `Open Box`, `Any`, and `Other`. | [optional]
**scheduled_delivery_start_date** | Option<**String**> | The start date of the scheduled delivery window in the time zone for the order destination. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date time format. | [optional]
**scheduled_delivery_end_date** | Option<**String**> | The end date of the scheduled delivery window in the time zone for the order destination. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date time format. | [optional]
**price_designation** | Option<**String**> | Indicates that the selling price is a special price that is only available for Amazon Business orders. For more information about the Amazon Business Seller Program, refer to the [Amazon Business website](https://www.amazon.com/b2b/info/amazon-business).   **Possible values**: `BusinessPrice` | [optional]
**tax_collection** | Option<[**models::TaxCollection**](TaxCollection.md)> |  | [optional]
**serial_number_required** | Option<**bool**> | When true, the product type for this item has a serial number.   Only returned for Amazon Easy Ship orders. | [optional]
**is_transparency** | Option<**bool**> | When true, the ASIN is enrolled in Transparency. The Transparency serial number that you must submit is determined by:  **1D or 2D Barcode:** This has a **T** logo. Submit either the 29-character alpha-numeric identifier beginning with **AZ** or **ZA**, or the 38-character Serialized Global Trade Item Number (SGTIN). **2D Barcode SN:** Submit the 7- to 20-character serial number barcode, which likely has the prefix **SN**. The serial number is applied to the same side of the packaging as the GTIN (UPC/EAN/ISBN) barcode. **QR code SN:** Submit the URL that the QR code generates. | [optional]
**ioss_number** | Option<**String**> | The IOSS number of the marketplace. Sellers shipping to the EU from outside the EU must provide this IOSS number to their carrier when Amazon has collected the VAT on the sale. | [optional]
**store_chain_store_id** | Option<**String**> | The store chain store identifier. Linked to a specific store in a store chain. | [optional]
**deemed_reseller_category** | Option<**String**> | The category of deemed reseller. This applies to selling partners that are not based in the EU and is used to help them meet the VAT Deemed Reseller tax laws in the EU and UK. | [optional]
**buyer_info** | Option<[**models::ItemBuyerInfo**](ItemBuyerInfo.md)> |  | [optional]
**buyer_requested_cancel** | Option<[**models::BuyerRequestedCancel**](BuyerRequestedCancel.md)> |  | [optional]
**serial_numbers** | Option<**Vec<String>**> | A list of serial numbers for electronic products that are shipped to customers. Returned for FBA orders only. | [optional]
**substitution_preferences** | Option<[**models::SubstitutionPreferences**](SubstitutionPreferences.md)> |  | [optional]
**measurement** | Option<[**models::Measurement**](Measurement.md)> |  | [optional]
**shipping_constraints** | Option<[**models::ShippingConstraints**](ShippingConstraints.md)> |  | [optional]
**amazon_programs** | Option<[**models::AmazonPrograms**](AmazonPrograms.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


