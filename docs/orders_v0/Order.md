# Order

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**amazon_order_id** | **String** | An Amazon-defined order identifier, in 3-7-7 format. | 
**seller_order_id** | Option<**String**> | A seller-defined order identifier. | [optional]
**purchase_date** | **String** | The date when the order was created. | 
**last_update_date** | **String** | The date when the order was last updated.  __Note__: `LastUpdateDate` is returned with an incorrect date for orders that were last updated before 2009-04-01. | 
**order_status** | **String** | The current order status. | 
**fulfillment_channel** | Option<**String**> | Whether the order was fulfilled by Amazon (`AFN`) or by the seller (`MFN`). | [optional]
**sales_channel** | Option<**String**> | The sales channel for the first item in the order. | [optional]
**order_channel** | Option<**String**> | The order channel for the first item in the order. | [optional]
**ship_service_level** | Option<**String**> | The order's shipment service level. | [optional]
**order_total** | Option<[**models::Money**](Money.md)> |  | [optional]
**number_of_items_shipped** | Option<**i32**> | The number of items shipped. | [optional]
**number_of_items_unshipped** | Option<**i32**> | The number of items unshipped. | [optional]
**payment_execution_detail** | Option<[**Vec<models::PaymentExecutionDetailItem>**](PaymentExecutionDetailItem.md)> | A list of payment execution detail items. | [optional]
**payment_method** | Option<**String**> | The payment method for the order. This property is limited to COD and CVS payment methods. Unless you need the specific COD payment information provided by the `PaymentExecutionDetailItem` object, we recommend using the `PaymentMethodDetails` property to get payment method information. | [optional]
**payment_method_details** | Option<**Vec<String>**> | A list of payment method detail items. | [optional]
**marketplace_id** | Option<**String**> | The identifier for the marketplace where the order was placed. | [optional]
**shipment_service_level_category** | Option<**String**> | The shipment service level category for the order.  **Possible values**: `Expedited`, `FreeEconomy`, `NextDay`, `Priority`, `SameDay`, `SecondDay`, `Scheduled`, and `Standard`. | [optional]
**easy_ship_shipment_status** | Option<[**models::EasyShipShipmentStatus**](EasyShipShipmentStatus.md)> |  | [optional]
**cba_displayable_shipping_label** | Option<**String**> | Custom ship label for Checkout by Amazon (CBA). | [optional]
**order_type** | Option<**String**> | The order's type. | [optional]
**earliest_ship_date** | Option<**String**> | The start of the time period within which you have committed to ship the order. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date time format. Only returned for seller-fulfilled orders.  __Note__: `EarliestShipDate` might not be returned for orders placed before February 1, 2013. | [optional]
**latest_ship_date** | Option<**String**> | The end of the time period within which you have committed to ship the order. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date time format. Only returned for seller-fulfilled orders.  __Note__: `LatestShipDate` might not be returned for orders placed before February 1, 2013. | [optional]
**earliest_delivery_date** | Option<**String**> | The start of the time period within which you have committed to fulfill the order. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date time format. Only returned for seller-fulfilled orders. | [optional]
**latest_delivery_date** | Option<**String**> | The end of the time period within which you have committed to fulfill the order. In [ISO 8601](https://developer-docs.amazon.com/sp-api/docs/iso-8601) date time format. Only returned for seller-fulfilled orders that do not have a `PendingAvailability`, `Pending`, or `Canceled` status. | [optional]
**is_business_order** | Option<**bool**> | When true, the order is an Amazon Business order. An Amazon Business order is an order where the buyer is a Verified Business Buyer. | [optional]
**is_prime** | Option<**bool**> | When true, the order is a seller-fulfilled Amazon Prime order. | [optional]
**is_premium_order** | Option<**bool**> | When true, the order has a Premium Shipping Service Level Agreement. For more information about Premium Shipping orders, refer to \"Premium Shipping Options\" in the Seller Central Help for your marketplace. | [optional]
**is_global_express_enabled** | Option<**bool**> | When true, the order is a `GlobalExpress` order. | [optional]
**replaced_order_id** | Option<**String**> | The order ID value for the order that is being replaced. Returned only if IsReplacementOrder = true. | [optional]
**is_replacement_order** | Option<**bool**> | When true, this is a replacement order. | [optional]
**promise_response_due_date** | Option<**String**> | Indicates the date by which the seller must respond to the buyer with an estimated ship date. Only returned for Sourcing on Demand orders. | [optional]
**is_estimated_ship_date_set** | Option<**bool**> | When true, the estimated ship date is set for the order. Only returned for Sourcing on Demand orders. | [optional]
**is_sold_by_ab** | Option<**bool**> | When true, the item within this order was bought and re-sold by Amazon Business EU SARL (ABEU). By buying and instantly re-selling your items, ABEU becomes the seller of record, making your inventory available for sale to customers who would not otherwise purchase from a third-party seller. | [optional]
**is_iba** | Option<**bool**> | When true, the item within this order was bought and re-sold by Amazon Business EU SARL (ABEU). By buying and instantly re-selling your items, ABEU becomes the seller of record, making your inventory available for sale to customers who would not otherwise purchase from a third-party seller. | [optional]
**default_ship_from_location_address** | Option<[**models::Address**](Address.md)> |  | [optional]
**buyer_invoice_preference** | Option<**String**> | The buyer's invoicing preference. Sellers can use this data to issue electronic invoices for orders in Turkey.  **Note**: This attribute is only available in the Turkey marketplace. | [optional]
**buyer_tax_information** | Option<[**models::BuyerTaxInformation**](BuyerTaxInformation.md)> |  | [optional]
**fulfillment_instruction** | Option<[**models::FulfillmentInstruction**](FulfillmentInstruction.md)> |  | [optional]
**is_ispu** | Option<**bool**> | When true, this order is marked to be picked up from a store rather than delivered. | [optional]
**is_access_point_order** | Option<**bool**> | When true, this order is marked to be delivered to an Access Point. The access location is chosen by the customer. Access Points include Amazon Hub Lockers, Amazon Hub Counters, and pickup points operated by carriers. | [optional]
**marketplace_tax_info** | Option<[**models::MarketplaceTaxInfo**](MarketplaceTaxInfo.md)> |  | [optional]
**seller_display_name** | Option<**String**> | The seller’s friendly name registered in the marketplace where the sale took place. Sellers can use this data to issue electronic invoices for orders in Brazil.  **Note**: This attribute is only available in the Brazil marketplace for the orders with `Pending` or `Unshipped` status. | [optional]
**shipping_address** | Option<[**models::Address**](Address.md)> |  | [optional]
**buyer_info** | Option<[**models::BuyerInfo**](BuyerInfo.md)> |  | [optional]
**automated_shipping_settings** | Option<[**models::AutomatedShippingSettings**](AutomatedShippingSettings.md)> |  | [optional]
**has_regulated_items** | Option<**bool**> | Whether the order contains regulated items which may require additional approval steps before being fulfilled. | [optional]
**electronic_invoice_status** | Option<[**models::ElectronicInvoiceStatus**](ElectronicInvoiceStatus.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


