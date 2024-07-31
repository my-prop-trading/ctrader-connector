use crate::manager::common_messages_external::ProtoErrorRes;
use crate::manager::common_model_messages_external::ProtoPayloadType;
use crate::manager::cs_messages_external::{ProtoCsPayloadType, ProtoManagerAuthRes};

#[derive(Debug)]
pub enum ManagerApiMessage {
    ErrorRes(ProtoErrorRes),
    HelloEvent,
    ManagerAuthRes(ProtoManagerAuthRes),
}

impl ManagerApiMessage {
    pub fn try_from_common(payload_type: i32, payload: &Option<Vec<u8>>) -> Option<Self> {
        let common_type = ProtoPayloadType::try_from(payload_type);

        if let Ok(common_type) = common_type {
            match common_type {
                ProtoPayloadType::ProtoMessage => {}
                ProtoPayloadType::ProtoServerDepthQuotesEvent => {}
                ProtoPayloadType::ProtoServerSpotEvent => {}
                ProtoPayloadType::ProtoServerDepthEvent => {}
                ProtoPayloadType::ProtoServerTrailingSlChangedEvent => {}
                ProtoPayloadType::ProtoServerMarketDataEvent => {}
                ProtoPayloadType::ErrorRes => {
                    let payload = payload.as_ref().unwrap();
                    return Some(ManagerApiMessage::ErrorRes(
                        prost::Message::decode(&payload[..]).unwrap(),
                    ));
                }
                ProtoPayloadType::HeartbeatEvent => {}
                ProtoPayloadType::RegisterCserverConnectionReq => {}
                ProtoPayloadType::RegisterCserverConnectionRes => {}
                ProtoPayloadType::UnregisterCserverConnectionReq => {}
                ProtoPayloadType::UnregisterCserverConnectionRes => {}
                ProtoPayloadType::RegisterCidConnectionReq => {}
                ProtoPayloadType::RegisterCidConnectionRes => {}
                ProtoPayloadType::UnregisterCidConnectionReq => {}
                ProtoPayloadType::UnregisterCidConnectionRes => {}
                ProtoPayloadType::AvailableServicesEvent => {}
                ProtoPayloadType::PingReq => {}
                ProtoPayloadType::PingRes => {}
            }
        }

        None
    }

    pub fn try_from_cs(payload_type: i32, payload: &Option<Vec<u8>>) -> Option<Self> {
        let cs_type = ProtoCsPayloadType::try_from(payload_type);

        if let Ok(cs_type) = cs_type {
            match cs_type {
                ProtoCsPayloadType::ProtoSpotEvent => {}
                ProtoCsPayloadType::ProtoTrendbarListReq => {}
                ProtoCsPayloadType::ProtoTrendbarListRes => {}
                ProtoCsPayloadType::ProtoOrderErrorEvent => {}
                ProtoCsPayloadType::ProtoVersionReq => {}
                ProtoCsPayloadType::ProtoVersionRes => {}
                ProtoCsPayloadType::ProtoManagerByIdReq => {}
                ProtoCsPayloadType::ProtoManagerByIdRes => {}
                ProtoCsPayloadType::ProtoManagerLightTraderListReq => {}
                ProtoCsPayloadType::ProtoManagerLightTraderListRes => {}
                ProtoCsPayloadType::ProtoExecutionEvent => {}
                ProtoCsPayloadType::ProtoManagerAuthReq => {}
                ProtoCsPayloadType::ProtoManagerAuthRes => {
                    let payload = payload.as_ref().unwrap();
                    return Some(ManagerApiMessage::ManagerAuthRes(
                        prost::Message::decode(&payload[..]).unwrap(),
                    ));
                }
                ProtoCsPayloadType::ProtoChangeTraderPasswordReq => {}
                ProtoCsPayloadType::ProtoChangeTraderPasswordRes => {}
                ProtoCsPayloadType::ProtoChangeManagerPasswordReq => {}
                ProtoCsPayloadType::ProtoChangeManagerPasswordRes => {}
                ProtoCsPayloadType::ProtoCheckTraderPasswordReq => {}
                ProtoCsPayloadType::ProtoCheckTraderPasswordRes => {}
                ProtoCsPayloadType::ProtoCheckManagerPasswordReq => {}
                ProtoCsPayloadType::ProtoCheckManagerPasswordRes => {}
                ProtoCsPayloadType::ProtoServerTimeReq => {}
                ProtoCsPayloadType::ProtoServerTimeRes => {}
                ProtoCsPayloadType::ProtoOrderDetailsReq => {}
                ProtoCsPayloadType::ProtoOrderDetailsRes => {}
                ProtoCsPayloadType::ProtoPositionMarginChangedEvent => {}
                ProtoCsPayloadType::ProtoRecalculateAccountMarginReq => {}
                ProtoCsPayloadType::ProtoRecalculateAccountMarginRes => {}
                ProtoCsPayloadType::ProtoRecalculateSymbolMarginReq => {}
                ProtoCsPayloadType::ProtoRecalculateSymbolMarginRes => {}
                ProtoCsPayloadType::ProtoRecalculateDynamicLeverageReq => {}
                ProtoCsPayloadType::ProtoRecalculateDynamicLeverageRes => {}
                ProtoCsPayloadType::ProtoManagerBalanceTransferReq => {}
                ProtoCsPayloadType::ProtoManagerBalanceTransferRes => {}
                ProtoCsPayloadType::ProtoCrudScheduleProfileReq => {}
                ProtoCsPayloadType::ProtoCrudScheduleProfileRes => {}
                ProtoCsPayloadType::ProtoScheduleProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoScheduleProfileListReq => {}
                ProtoCsPayloadType::ProtoScheduleProfileListRes => {}
                ProtoCsPayloadType::ProtoCrudCommissionProfileReq => {}
                ProtoCsPayloadType::ProtoCrudCommissionProfileRes => {}
                ProtoCsPayloadType::ProtoCommissionProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoCommissionProfileListReq => {}
                ProtoCsPayloadType::ProtoCommissionProfileListRes => {}
                ProtoCsPayloadType::ProtoCrudVolumeProfileReq => {}
                ProtoCsPayloadType::ProtoCrudVolumeProfileRes => {}
                ProtoCsPayloadType::ProtoVolumeProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoVolumeProfileListReq => {}
                ProtoCsPayloadType::ProtoVolumeProfileListRes => {}
                ProtoCsPayloadType::ProtoCrudExecutionProfileReq => {}
                ProtoCsPayloadType::ProtoCrudExecutionProfileRes => {}
                ProtoCsPayloadType::ProtoExecutionProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoExecutionProfileListReq => {}
                ProtoCsPayloadType::ProtoExecutionProfileListRes => {}
                ProtoCsPayloadType::ProtoCrudProtectionProfileReq => {}
                ProtoCsPayloadType::ProtoCrudProtectionProfileRes => {}
                ProtoCsPayloadType::ProtoProtectionProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoProtectionProfileListReq => {}
                ProtoCsPayloadType::ProtoProtectionProfileListRes => {}
                ProtoCsPayloadType::ProtoCrudSwapFreeProfileReq => {}
                ProtoCsPayloadType::ProtoCrudSwapFreeProfileRes => {}
                ProtoCsPayloadType::ProtoSwapFreeProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoSwapFreeProfileListReq => {}
                ProtoCsPayloadType::ProtoSwapFreeProfileListRes => {}
                ProtoCsPayloadType::ProtoCrudHolidayReq => {}
                ProtoCsPayloadType::ProtoCrudHolidayRes => {}
                ProtoCsPayloadType::ProtoHolidayChangedEvent => {}
                ProtoCsPayloadType::ProtoHolidayListReq => {}
                ProtoCsPayloadType::ProtoHolidayListRes => {}
                ProtoCsPayloadType::ProtoCrudHolidayProfileReq => {}
                ProtoCsPayloadType::ProtoCrudHolidayProfileRes => {}
                ProtoCsPayloadType::ProtoHolidayProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoTraderListReq => {}
                ProtoCsPayloadType::ProtoTraderListRes => {}
                ProtoCsPayloadType::ProtoPositionListReq => {}
                ProtoCsPayloadType::ProtoPositionListRes => {}
                ProtoCsPayloadType::ProtoPendingOrderListReq => {}
                ProtoCsPayloadType::ProtoPendingOrderListRes => {}
                ProtoCsPayloadType::ProtoManagerListReq => {}
                ProtoCsPayloadType::ProtoManagerListRes => {}
                ProtoCsPayloadType::ProtoBalanceHistoryListReq => {}
                ProtoCsPayloadType::ProtoBalanceHistoryListRes => {}
                ProtoCsPayloadType::ProtoExposureSymbolListReq => {}
                ProtoCsPayloadType::ProtoExposureSymbolListRes => {}
                ProtoCsPayloadType::ProtoServerSettingsReq => {}
                ProtoCsPayloadType::ProtoServerSettingsRes => {}
                ProtoCsPayloadType::ProtoPriceStreamListReq => {}
                ProtoCsPayloadType::ProtoPriceStreamListRes => {}
                ProtoCsPayloadType::ProtoLiquidityFeedListReq => {}
                ProtoCsPayloadType::ProtoLiquidityFeedListRes => {}
                ProtoCsPayloadType::ProtoManagerDealListReq => {}
                ProtoCsPayloadType::ProtoManagerDealListRes => {}
                ProtoCsPayloadType::ProtoCountryListReq => {}
                ProtoCsPayloadType::ProtoCountryListRes => {}
                ProtoCsPayloadType::ProtoAssetClassListReq => {}
                ProtoCsPayloadType::ProtoAssetClassListRes => {}
                ProtoCsPayloadType::ProtoOrderManagerListReq => {}
                ProtoCsPayloadType::ProtoOrderManagerListRes => {}
                ProtoCsPayloadType::ProtoHolidayProfileListReq => {}
                ProtoCsPayloadType::ProtoHolidayProfileListRes => {}
                ProtoCsPayloadType::ProtoManagerDealListByPositionIdReq => {}
                ProtoCsPayloadType::ProtoManagerDealListByPositionIdRes => {}
                ProtoCsPayloadType::ProtoManagerOrderListByPositionIdReq => {}
                ProtoCsPayloadType::ProtoManagerOrderListByPositionIdRes => {}
                ProtoCsPayloadType::ProtoSymbolCategoryListReq => {}
                ProtoCsPayloadType::ProtoSymbolCategoryListRes => {}
                ProtoCsPayloadType::ProtoAssetListReq => {}
                ProtoCsPayloadType::ProtoAssetListRes => {}
                ProtoCsPayloadType::ProtoManagerSymbolListReq => {}
                ProtoCsPayloadType::ProtoManagerSymbolListRes => {}
                ProtoCsPayloadType::ProtoDynamicLeverageListReq => {}
                ProtoCsPayloadType::ProtoDynamicLeverageListRes => {}
                ProtoCsPayloadType::ProtoGslScheduleListReq => {}
                ProtoCsPayloadType::ProtoGslScheduleListRes => {}
                ProtoCsPayloadType::ProtoLightGroupListReq => {}
                ProtoCsPayloadType::ProtoLightGroupListRes => {}
                ProtoCsPayloadType::ProtoGroupByIdReq => {}
                ProtoCsPayloadType::ProtoGroupByIdRes => {}
                ProtoCsPayloadType::ProtoLiquidityFeedSymbolListReq => {}
                ProtoCsPayloadType::ProtoLiquidityFeedSymbolListRes => {}
                ProtoCsPayloadType::ProtoLightSwapAndDividendProfileListReq => {}
                ProtoCsPayloadType::ProtoLightSwapAndDividendProfileListRes => {}
                ProtoCsPayloadType::ProtoSwapAndDividendProfileByIdReq => {}
                ProtoCsPayloadType::ProtoSwapAndDividendProfileByIdRes => {}
                ProtoCsPayloadType::ProtoCrudTraderReq => {}
                ProtoCsPayloadType::ProtoCrudTraderRes => {}
                ProtoCsPayloadType::ProtoTraderChangedEvent => {}
                ProtoCsPayloadType::ProtoCrudGroupReq => {}
                ProtoCsPayloadType::ProtoCrudGroupRes => {}
                ProtoCsPayloadType::ProtoGroupChangedEvent => {}
                ProtoCsPayloadType::ProtoCrudSymbolReq => {}
                ProtoCsPayloadType::ProtoCrudSymbolRes => {}
                ProtoCsPayloadType::ProtoCrudManagerReq => {}
                ProtoCsPayloadType::ProtoCrudManagerRes => {}
                ProtoCsPayloadType::ProtoManagerChangedEvent => {}
                ProtoCsPayloadType::ProtoCrudSwapAndDividendProfileReq => {}
                ProtoCsPayloadType::ProtoCrudSwapAndDividendProfileRes => {}
                ProtoCsPayloadType::ProtoSwapAndDividendProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoUpdateServerSettingsReq => {}
                ProtoCsPayloadType::ProtoUpdateServerSettingsRes => {}
                ProtoCsPayloadType::ProtoServerSettingsChangedEvent => {}
                ProtoCsPayloadType::ProtoChangeBalanceReq => {}
                ProtoCsPayloadType::ProtoChangeBalanceRes => {}
                ProtoCsPayloadType::ProtoPriceStreamCreateReq => {}
                ProtoCsPayloadType::ProtoPriceStreamCreateRes => {}
                ProtoCsPayloadType::ProtoPriceStreamDeleteReq => {}
                ProtoCsPayloadType::ProtoPriceStreamDeleteRes => {}
                ProtoCsPayloadType::ProtoPriceStreamUpdateReq => {}
                ProtoCsPayloadType::ProtoPriceStreamUpdateRes => {}
                ProtoCsPayloadType::ProtoPriceStreamChangedEvent => {}
                ProtoCsPayloadType::ProtoCrudAssetReq => {}
                ProtoCsPayloadType::ProtoCrudAssetRes => {}
                ProtoCsPayloadType::ProtoAssetChangedEvent => {}
                ProtoCsPayloadType::ProtoCrudLiquidityFeedSymbolReq => {}
                ProtoCsPayloadType::ProtoCrudLiquidityFeedSymbolRes => {}
                ProtoCsPayloadType::ProtoLiquidityFeedSymbolChangedEvent => {}
                ProtoCsPayloadType::ProtoInsertTrendbarReq => {}
                ProtoCsPayloadType::ProtoInsertTrendbarRes => {}
                ProtoCsPayloadType::ProtoManagerSymbolChangedEvent => {}
                ProtoCsPayloadType::ProtoCrudDynamicLeverageReq => {}
                ProtoCsPayloadType::ProtoCrudDynamicLeverageRes => {}
                ProtoCsPayloadType::ProtoDynamicLeverageChangedEvent => {}
                ProtoCsPayloadType::ProtoTraderPermissionLoseEvent => {}
                ProtoCsPayloadType::ProtoCrudGslScheduleReq => {}
                ProtoCsPayloadType::ProtoCrudGslScheduleRes => {}
                ProtoCsPayloadType::ProtoGslScheduleChangedEvent => {}
                ProtoCsPayloadType::ProtoCreateSymbolReq => {}
                ProtoCsPayloadType::ProtoCreateSymbolRes => {}
                ProtoCsPayloadType::ProtoSymbolArchivedEvent => {}
                ProtoCsPayloadType::ProtoSymbolRestoredEvent => {}
                ProtoCsPayloadType::ProtoCrudTradeNotificationProfileReq => {}
                ProtoCsPayloadType::ProtoCrudTradeNotificationProfileRes => {}
                ProtoCsPayloadType::ProtoCrudTradeNotificationProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoCrudTradeNotificationProfileListReq => {}
                ProtoCsPayloadType::ProtoCrudTradeNotificationProfileListRes => {}
                ProtoCsPayloadType::ProtoDeleteTrendbarReq => {}
                ProtoCsPayloadType::ProtoDeleteTrendbarRes => {}
                ProtoCsPayloadType::ProtoSubscribeSpotQuotesReq => {}
                ProtoCsPayloadType::ProtoSubscribeSpotQuotesRes => {}
                ProtoCsPayloadType::ProtoUnsubscribeSpotQuotesReq => {}
                ProtoCsPayloadType::ProtoUnsubscribeSpotQuotesRes => {}
                ProtoCsPayloadType::ProtoTraderByIdReq => {}
                ProtoCsPayloadType::ProtoTraderByIdRes => {}
                ProtoCsPayloadType::ProtoManagerGetDealReq => {}
                ProtoCsPayloadType::ProtoManagerGetDealRes => {}
                ProtoCsPayloadType::ProtoManagerClosedPositionListReq => {}
                ProtoCsPayloadType::ProtoManagerClosedPositionListRes => {}
                ProtoCsPayloadType::ProtoTraderLogonEvent => {}
                ProtoCsPayloadType::ProtoTraderLogoutEvent => {}
                ProtoCsPayloadType::ProtoManagerNewOrderReq => {}
                ProtoCsPayloadType::ProtoManagerAmendOrderReq => {}
                ProtoCsPayloadType::ProtoManagerCancelOrderReq => {}
                ProtoCsPayloadType::ProtoManagerAmendPositionReq => {}
                ProtoCsPayloadType::ProtoManagerClosePositionReq => {}
                ProtoCsPayloadType::ProtoRebuildTrendbarsReq => {}
                ProtoCsPayloadType::ProtoRebuildTrendbarsRes => {}
                ProtoCsPayloadType::ProtoPositionDetailsLiteReq => {}
                ProtoCsPayloadType::ProtoPositionDetailsLiteRes => {}
                ProtoCsPayloadType::ProtoManagerChangeBonusReq => {}
                ProtoCsPayloadType::ProtoManagerChangeBonusRes => {}
                ProtoCsPayloadType::ProtoBonusHistoryListReq => {}
                ProtoCsPayloadType::ProtoBonusHistoryListRes => {}
                ProtoCsPayloadType::ProtoLiquidityFeedStatusReq => {}
                ProtoCsPayloadType::ProtoLiquidityFeedStatusRes => {}
                ProtoCsPayloadType::ProtoDealingSettingsReq => {}
                ProtoCsPayloadType::ProtoDealingSettingsRes => {}
                ProtoCsPayloadType::ProtoUpdateDealingSettingsReq => {}
                ProtoCsPayloadType::ProtoUpdateDealingSettingsRes => {}
                ProtoCsPayloadType::ProtoNewManualDealEvent => {}
                ProtoCsPayloadType::ProtoManualDealListReq => {}
                ProtoCsPayloadType::ProtoManualDealListRes => {}
                ProtoCsPayloadType::ProtoManualDealClaimReq => {}
                ProtoCsPayloadType::ProtoManualDealClaimRes => {}
                ProtoCsPayloadType::ProtoManualDealClaimedEvent => {}
                ProtoCsPayloadType::ProtoManualDealUnclaimReq => {}
                ProtoCsPayloadType::ProtoManualDealUnclaimRes => {}
                ProtoCsPayloadType::ProtoManualDealUnclaimedEvent => {}
                ProtoCsPayloadType::ProtoManualDealResetReq => {}
                ProtoCsPayloadType::ProtoManualDealResetRes => {}
                ProtoCsPayloadType::ProtoManualDealRejectReq => {}
                ProtoCsPayloadType::ProtoManualDealRejectRes => {}
                ProtoCsPayloadType::ProtoManualDealExecuteReq => {}
                ProtoCsPayloadType::ProtoManualDealExecuteRes => {}
                ProtoCsPayloadType::ProtoManualDealProcessedEvent => {}
                ProtoCsPayloadType::ProtoDealerNewOrderReq => {}
                ProtoCsPayloadType::ProtoDealerAmendOrderReq => {}
                ProtoCsPayloadType::ProtoDealerCancelOrderReq => {}
                ProtoCsPayloadType::ProtoDealerAmendPositionReq => {}
                ProtoCsPayloadType::ProtoDealerClosePositionReq => {}
                ProtoCsPayloadType::ProtoDealingSettingsUpdatedEvent => {}
                ProtoCsPayloadType::ProtoAssetClassChangedEvent => {}
                ProtoCsPayloadType::ProtoAssetClassDeletedEvent => {}
                ProtoCsPayloadType::ProtoSymbolCategoryChangedEvent => {}
                ProtoCsPayloadType::ProtoSymbolCategoryDeletedEvent => {}
                ProtoCsPayloadType::ProtoManagerGetAuthTokenReq => {}
                ProtoCsPayloadType::ProtoManagerGetAuthTokenRes => {}
                ProtoCsPayloadType::ProtoSymbolsForConversionReq => {}
                ProtoCsPayloadType::ProtoSymbolsForConversionRes => {}
                ProtoCsPayloadType::ProtoForceClosePositionReq => {}
                ProtoCsPayloadType::ProtoForceOpenPositionReq => {}
                ProtoCsPayloadType::ProtoCrudMaxAutoExecutionSizeProfileReq => {}
                ProtoCsPayloadType::ProtoCrudMaxAutoExecutionSizeProfileRes => {}
                ProtoCsPayloadType::ProtoCrudMaxAutoExecutionSizeProfileChangedEvent => {}
                ProtoCsPayloadType::ProtoCrudMaxAutoExecutionSizeProfileListReq => {}
                ProtoCsPayloadType::ProtoCrudMaxAutoExecutionSizeProfileListRes => {}
                ProtoCsPayloadType::ProtoHelloEvent => return Some(ManagerApiMessage::HelloEvent),
            }
        }

        None
    }
}
