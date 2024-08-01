use crate::manager::common_messages_external::{ProtoErrorRes, ProtoMessage};
use crate::manager::cs_messages_external::{ProtoCsPayloadType, ProtoManagerAuthRes};

#[derive(Debug)]
pub enum ManagerApiMessage {
    ErrorRes(ProtoErrorRes),
    HelloEvent,
    ManagerAuthRes(ProtoManagerAuthRes),
    HeartbeatEvent,
}

impl ManagerApiMessage {
    pub fn try_from_proto(proto: ProtoMessage) -> Result<Option<Self>, String> {
        let payload = proto.payload;
        let payload_type =
            ProtoCsPayloadType::try_from(proto.payload_type as i32).expect("must be valid proto");

        match payload_type {
            ProtoCsPayloadType::ProtoMessage => {}
            ProtoCsPayloadType::ProtoServerDepthQuotesEvent => {}
            //ProtoPayloadType::ProtoServerSpotEvent => {}
            ProtoCsPayloadType::ProtoServerDepthEvent => {}
            ProtoCsPayloadType::ProtoServerTrailingSlChangedEvent => {}
            ProtoCsPayloadType::ProtoServerMarketDataEvent => {}
            ProtoCsPayloadType::ErrorRes => {
                let payload = payload.as_ref().unwrap();
                return Ok(Some(ManagerApiMessage::ErrorRes(
                    prost::Message::decode(&payload[..]).unwrap(),
                )));
            }
            ProtoCsPayloadType::HeartbeatEvent => {
                return Ok(Some(ManagerApiMessage::HeartbeatEvent))
            }
            ProtoCsPayloadType::RegisterCserverConnectionReq => {}
            ProtoCsPayloadType::RegisterCserverConnectionRes => {}
            ProtoCsPayloadType::UnregisterCserverConnectionReq => {}
            ProtoCsPayloadType::UnregisterCserverConnectionRes => {}
            ProtoCsPayloadType::RegisterCidConnectionReq => {}
            ProtoCsPayloadType::RegisterCidConnectionRes => {}
            ProtoCsPayloadType::UnregisterCidConnectionReq => {}
            ProtoCsPayloadType::UnregisterCidConnectionRes => {}
            ProtoCsPayloadType::AvailableServicesEvent => {}
            ProtoCsPayloadType::PingReq => {}
            ProtoCsPayloadType::PingRes => return Ok(None),
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
                return Ok(Some(ManagerApiMessage::ManagerAuthRes(
                    prost::Message::decode(&payload[..]).unwrap(),
                )));
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
            ProtoCsPayloadType::ProtoHelloEvent => return Ok(Some(ManagerApiMessage::HelloEvent)),
        }

        Err(format!(
            "Payload type {} is not implemented",
            payload_type.as_str_name()
        ))
    }
}
