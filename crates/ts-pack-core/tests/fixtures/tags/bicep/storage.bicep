targetScope = 'resourceGroup'

@description('Storage account name prefix')
param prefix string = 'demo'

var location = resourceGroup().location

type skuName = 'Standard_LRS' | 'Premium_LRS'

func accountName(base string) string => '${base}${uniqueString(resourceGroup().id)}'

resource account 'Microsoft.Storage/storageAccounts@2023-01-01' = {
  name: accountName(prefix)
  location: location
  kind: 'StorageV2'
  sku: {
    name: 'Standard_LRS'
  }
}

module network './network.bicep' = {
  name: 'network'
}

output accountId string = account.id
