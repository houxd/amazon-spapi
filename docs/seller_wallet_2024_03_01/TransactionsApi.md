# \TransactionsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_transaction**](TransactionsApi.md#create_transaction) | **POST** /finances/transfers/wallet/2024-03-01/transactions | Create a transaction request from Amazon SW account to another customer provided account
[**get_transaction**](TransactionsApi.md#get_transaction) | **GET** /finances/transfers/wallet/2024-03-01/transactions/{transactionId} | Find particular Amazon SW account transaction by Amazon transaction identifier
[**list_account_transactions**](TransactionsApi.md#list_account_transactions) | **GET** /finances/transfers/wallet/2024-03-01/transactions | The API will return all the transactions for a given Amazon SW account sorted by the transaction request date



## create_transaction

> models::Transaction create_transaction(dest_account_digital_signature, amount_digital_signature, body)
Create a transaction request from Amazon SW account to another customer provided account

Create a transaction request from a Seller Wallet account to another customer-provided account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**dest_account_digital_signature** | **String** | Digital signature for the destination bank account details. | [required] |
**amount_digital_signature** | **String** | Digital signature for the source currency transaction amount. | [required] |
**body** | [**TransactionInitiationRequest**](TransactionInitiationRequest.md) | Defines the actual payload of the request | [required] |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction

> models::Transaction get_transaction(transaction_id)
Find particular Amazon SW account transaction by Amazon transaction identifier

Returns a transaction

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**transaction_id** | **String** | ID of the Amazon SW transaction | [required] |

### Return type

[**models::Transaction**](Transaction.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_account_transactions

> models::TransactionListing list_account_transactions(account_id, next_page_token)
The API will return all the transactions for a given Amazon SW account sorted by the transaction request date

Retrieve a list of transactions for a given Seller Wallet bank account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the Amazon SW account | [required] |
**next_page_token** | Option<**String**> | Pagination token to retrieve a specific page of results. |  |

### Return type

[**models::TransactionListing**](TransactionListing.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

