# \AccountsApi

All URIs are relative to *https://sellingpartnerapi-na.amazon.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_account**](AccountsApi.md#get_account) | **GET** /finances/transfers/wallet/2024-03-01/accounts/{accountId} | Find particular Amazon SW account by Amazon account identifier
[**list_account_balances**](AccountsApi.md#list_account_balances) | **GET** /finances/transfers/wallet/2024-03-01/accounts/{accountId}/balance | Find balance in particular Amazon SW account by Amazon account identifier
[**list_accounts**](AccountsApi.md#list_accounts) | **GET** /finances/transfers/wallet/2024-03-01/accounts | Get all Amazon SW accounts for the seller



## get_account

> models::BankAccount get_account(account_id)
Find particular Amazon SW account by Amazon account identifier

Retrieve a Seller Wallet bank account by Amazon account identifier.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the Amazon SW account | [required] |

### Return type

[**models::BankAccount**](BankAccount.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_account_balances

> models::BalanceListing list_account_balances(account_id)
Find balance in particular Amazon SW account by Amazon account identifier

Retrieve the balance in a given Seller Wallet bank account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the Amazon SW account | [required] |

### Return type

[**models::BalanceListing**](BalanceListing.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_accounts

> models::BankAccountListing list_accounts(marketplace_id)
Get all Amazon SW accounts for the seller

Get Seller Wallet accounts for a seller.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**marketplace_id** | **String** | A marketplace identifier. Specifies the marketplace for which items are returned. | [required] |

### Return type

[**models::BankAccountListing**](BankAccountListing.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

