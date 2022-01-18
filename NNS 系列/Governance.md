# NNS 系列之 Governance

# Contents
- [NNS 系列之 Governance](#nns-系列之-governance)
- [Contents](#contents)
  - [概述](#概述)
  - [数据结构](#数据结构)
    - [GOVERNANCE](#governance)
  - [接口函数](#接口函数)
    - [canister_init](#canister_init)
    - [canister_pre_upgrade](#canister_pre_upgrade)
    - [canister_post_upgrade](#canister_post_upgrade)
    - [update_authz](#update_authz)
    - [current_authz](#current_authz)
    - [forward_vote](#forward_vote)
    - [transaction_notification](#transaction_notification)
    - [transaction_notification_pb](#transaction_notification_pb)
    - [claim_or_refresh_neuron_from_account](#claim_or_refresh_neuron_from_account)
    - [claim_gtc_neurons](#claim_gtc_neurons)
    - [transfer_gtc_neuron](#transfer_gtc_neuron)
    - [manage_neuron](#manage_neuron)
    - [get_full_neuron_by_id_or_subaccount](#get_full_neuron_by_id_or_subaccount)
    - [get_full_neuron](#get_full_neuron)
    - [get_neuron_info](#get_neuron_info)
    - [get_neuron_info_by_id_or_subaccount](#get_neuron_info_by_id_or_subaccount)
    - [get_proposal_info](#get_proposal_info)
    - [get_pending_proposals](#get_pending_proposals)
    - [list_proposals](#list_proposals)
    - [list_neurons](#list_neurons)
    - [get_monthly_node_provider_rewards](#get_monthly_node_provider_rewards)
    - [submit_proposal](#submit_proposal)
    - [execute_eligible_proposals](#execute_eligible_proposals)
    - [get_latest_reward_event](#get_latest_reward_event)
    - [get_neuron_ids](#get_neuron_ids)
    - [canister_heartbeat](#canister_heartbeat)
    - [manage_neuron_pb](#manage_neuron_pb)
    - [claim_or_refresh_neuron_from_account_pb](#claim_or_refresh_neuron_from_account_pb)
    - [list_proposals_pb](#list_proposals_pb)
    - [list_neurons_pb](#list_neurons_pb)
    - [update_node_provider](#update_node_provider)
    - [http_request](#http_request)
    - [__get_candid_interface_tmp_hack](#__get_candid_interface_tmp_hack)


## 概述

Governance canister，功能上是治理。里面会管理神经元，以及投票相关的功能。

## 数据结构

### GOVERNANCE

## 接口函数

### canister_init

### canister_pre_upgrade

### canister_post_upgrade

### update_authz

### current_authz

### forward_vote

```rust
manage_neuron_(ManageNeuron)
```

### transaction_notification

Please use ManageNeuron::ClaimOrRefresh. 

### transaction_notification_pb

Please use ManageNeuron::ClaimOrRefresh. 

### claim_or_refresh_neuron_from_account

claim_or_refresh_neuron_from_account_
manage_neuron_

Please use ManageNeuron::ClaimOrRefresh. 

### claim_gtc_neurons
```rust
fn claim_gtc_neurons_()
  governance_mut().claim_gtc_neurons()


```

### transfer_gtc_neuron
governance_mut()
        .transfer_gtc_neuron(
### manage_neuron

manage_neuron_(ManageNeuron)

### get_full_neuron_by_id_or_subaccount
```rust
get_full_neuron_by_id_or_subaccount_

fn get_full_neuron_by_id_or_subaccount_(
    by: NeuronIdOrSubaccount,
) -> Result<Neuron, GovernanceError> {
    governance().get_full_neuron_by_id_or_subaccount(&by, &caller())
}
```
### get_full_neuron

```rust
fn get_full_neuron_(neuron_id: NeuronId) -> Result<Neuron, GovernanceError> {
    governance().get_full_neuron(&NeuronIdProto::from(neuron_id), &caller())
}

```
### get_neuron_info
```rust
fn get_neuron_info_(neuron_id: NeuronId) -> Result<NeuronInfo, GovernanceError> {
    governance().get_neuron_info(&NeuronIdProto::from(neuron_id))
}
```
### get_neuron_info_by_id_or_subaccount

### get_proposal_info

### get_pending_proposals

### list_proposals

### list_neurons

### get_monthly_node_provider_rewards

### submit_proposal

### execute_eligible_proposals

### get_latest_reward_event

### get_neuron_ids

### canister_heartbeat

### manage_neuron_pb

### claim_or_refresh_neuron_from_account_pb

### list_proposals_pb

### list_neurons_pb

### update_node_provider

### http_request

### __get_candid_interface_tmp_hack



