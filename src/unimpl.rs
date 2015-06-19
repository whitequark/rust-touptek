    fn Toupcam_StartPullModeWithWndMsg(h: *mut Handle, IntPtr hWnd, uint nMsg) -> HRESULT;
    fn Toupcam_StartPullModeWithCallback(h: *mut Handle, PTOUPCAM_EVENT_CALLBACK pEventCallback, IntPtr pCallbackCtx) -> HRESULT;
    fn Toupcam_PullImage(h: *mut Handle, IntPtr pImageData, int bits, out uint pnWidth, out uint pnHeight) -> HRESULT;
    fn Toupcam_PullStillImage(h: *mut Handle, IntPtr pImageData, int bits, out uint pnWidth, out uint pnHeight) -> HRESULT;
    fn Toupcam_StartPushMode(h: *mut Handle, PTOUPCAM_DATA_CALLBACK pDataCallback, IntPtr pCallbackCtx) -> HRESULT;
    fn Toupcam_Stop(h: *mut Handle) -> HRESULT;
    fn Toupcam_Pause(h: *mut Handle, int bPause) -> HRESULT;

    /* for still image snap */
    fn Toupcam_Snap(h: *mut Handle, uint nResolutionIndex) -> HRESULT;

    /* for soft trigger */
    fn Toupcam_Trigger(h: *mut Handle) -> HRESULT;

    /*
        put_Size, put_eSize, can be used to set the video output resolution BEFORE Start.
        put_Size use width and height parameters, put_eSize use the index parameter.
        for example, UCMOS03100KPA support the following resolutions:
            index 0:    2048,   1536
            index 1:    1024,   768
            index 2:    680,    510
        so, we can use put_Size(h, 1024, 768) or put_eSize(h, 1). Both have the same effect.
    */
    fn Toupcam_put_Size(h: *mut Handle, int nWidth, int nHeight) -> HRESULT;
    fn Toupcam_get_Size(h: *mut Handle, out int nWidth, out int nHeight) -> HRESULT;
    fn Toupcam_put_eSize(h: *mut Handle, uint nResolutionIndex) -> HRESULT;
    fn Toupcam_get_eSize(h: *mut Handle, out uint nResolutionIndex) -> HRESULT;
    fn Toupcam_get_ResolutionNumber(h: *mut Handle) -> uint;
    fn Toupcam_get_Resolution(h: *mut Handle, uint nResolutionIndex, out int pWidth, out int pHeight) -> uint;
    fn Toupcam_get_ResolutionRatio(h: *mut Handle, uint nResolutionIndex, out int pNumerator, out int pDenominator) -> uint;

    /*
        FourCC:
            MAKEFOURCC('G', 'B', 'R', 'G')
            MAKEFOURCC('R', 'G', 'G', 'B')
            MAKEFOURCC('B', 'G', 'G', 'R')
            MAKEFOURCC('G', 'R', 'B', 'G')
            MAKEFOURCC('Y', 'U', 'Y', 'V')
            MAKEFOURCC('Y', 'Y', 'Y', 'Y')
    */
    fn Toupcam_get_RawFormat(h: *mut Handle, out uint nFourCC, out uint bitdepth) -> uint;

    fn Toupcam_put_RealTime(h: *mut Handle, int bEnable) -> HRESULT;
    fn Toupcam_get_RealTime(h: *mut Handle, out int bEnable) -> HRESULT;
    fn Toupcam_Flush(h: *mut Handle) -> HRESULT;

    /* sensor Temperature */
    fn Toupcam_get_Temperature(h: *mut Handle, out short pTemperature) -> HRESULT;
    fn Toupcam_put_Temperature(h: *mut Handle, short nTemperature) -> HRESULT;

    /* ROI */
    fn Toupcam_get_Roi(h: *mut Handle, out uint pxOffset, out uint pyOffset, out uint pxWidth, out uint pyHeight) -> HRESULT;
    fn Toupcam_put_Roi(h: *mut Handle, uint xOffset, uint yOffset, uint xWidth, uint yHeight) -> HRESULT;

    /*
        ------------------------------------------------------------------|
        | Parameter               |   Range       |   Default             |
        |-----------------------------------------------------------------|
        | Auto Exposure Target    |   16~235      |   120                 |
        | Temp                    |   2000~15000  |   6503                |
        | Tint                    |   200~2500    |   1000                |
        | LevelRange              |   0~255       |   Low = 0, High = 255 |
        | Contrast                |   -100~100    |   0                   |
        | Hue                     |   -180~180    |   0                   |
        | Saturation              |   0~255       |   128                 |
        | Brightness              |   -64~64      |   0                   |
        | Gamma                   |   20~180      |   100                 |
        | WBGain                  |   -128~128    |   0                   |
        ------------------------------------------------------------------|
    */
    fn Toupcam_get_AutoExpoEnable(h: *mut Handle, out int bAutoExposure) -> HRESULT;
    fn Toupcam_put_AutoExpoEnable(h: *mut Handle, int bAutoExposure) -> HRESULT;
    fn Toupcam_get_AutoExpoTarget(h: *mut Handle, out ushort Target) -> HRESULT;
    fn Toupcam_put_AutoExpoTarget(h: *mut Handle, ushort Target) -> HRESULT;
    fn Toupcam_put_MaxAutoExpoTimeAGain(h: *mut Handle, uint maxTime, ushort maxAGain) -> HRESULT;

    /* in microseconds */
    fn Toupcam_get_ExpoTime(h: *mut Handle, out uint Time) -> HRESULT;
    /* in microseconds */
    fn Toupcam_put_ExpoTime(h: *mut Handle, uint Time) -> HRESULT;
    fn Toupcam_get_ExpTimeRange(h: *mut Handle, out uint nMin, out uint nMax, out uint nDef) -> HRESULT;

    /* percent, such as 300 */
    fn Toupcam_get_ExpoAGain(h: *mut Handle, out ushort AGain) -> HRESULT;
    /* percent */
    fn Toupcam_put_ExpoAGain(h: *mut Handle, ushort AGain) -> HRESULT;
    fn Toupcam_get_ExpoAGainRange(h: *mut Handle, out ushort nMin, out ushort nMax, out ushort nDef) -> HRESULT;

    fn Toupcam_put_LevelRange(h: *mut Handle, [In] ushort[] aLow, [In] ushort[] aHigh) -> HRESULT;
    fn Toupcam_get_LevelRange(h: *mut Handle, [Out] ushort[] aLow, [Out] ushort[] aHigh) -> HRESULT;

    fn Toupcam_put_Hue(h: *mut Handle, int Hue) -> HRESULT;
    fn Toupcam_get_Hue(h: *mut Handle, out int Hue) -> HRESULT;
    fn Toupcam_put_Saturation(h: *mut Handle, int Saturation) -> HRESULT;
    fn Toupcam_get_Saturation(h: *mut Handle, out int Saturation) -> HRESULT;
    fn Toupcam_put_Brightness(h: *mut Handle, int Brightness) -> HRESULT;
    fn Toupcam_get_Brightness(h: *mut Handle, out int Brightness) -> HRESULT;
    fn Toupcam_get_Contrast(h: *mut Handle, out int Contrast) -> HRESULT;
    fn Toupcam_put_Contrast(h: *mut Handle, int Contrast) -> HRESULT;
    fn Toupcam_get_Gamma(h: *mut Handle, out int Gamma) -> HRESULT;
    fn Toupcam_put_Gamma(h: *mut Handle, int Gamma) -> HRESULT;

    /* monochromatic mode */
    fn Toupcam_get_Chrome(h: *mut Handle, out int bChrome) -> HRESULT;
    fn Toupcam_put_Chrome(h: *mut Handle, int bChrome) -> HRESULT;

    fn Toupcam_get_VFlip(h: *mut Handle, out int bVFlip) -> HRESULT;
    fn Toupcam_put_VFlip(h: *mut Handle, int bVFlip) -> HRESULT;
    fn Toupcam_get_HFlip(h: *mut Handle, out int bHFlip) -> HRESULT;
    fn Toupcam_put_HFlip(h: *mut Handle, int bHFlip) -> HRESULT;

    fn Toupcam_get_Negative(h: *mut Handle, out int bNegative) -> HRESULT;
    fn Toupcam_put_Negative(h: *mut Handle, int bNegative) -> HRESULT;

    fn Toupcam_put_Speed(h: *mut Handle, ushort nSpeed) -> HRESULT;
    fn Toupcam_get_Speed(h: *mut Handle, out ushort pSpeed) -> HRESULT;
    /* get the maximum speed, "Frame Speed Level", speed range = [0, max] */
    fn Toupcam_get_MaxSpeed(h: *mut Handle) -> uint;

    /* get the max bit depth of this camera, such as 8, 10, 12, 14, 16 */
    fn Toupcam_get_MaxBitDepth(h: *mut Handle) -> uint;

    /* power supply:
            0 -> 60HZ AC
            1 -> 50Hz AC
            2 -> DC
    */
    fn Toupcam_put_HZ(h: *mut Handle, int nHZ) -> HRESULT;
    fn Toupcam_get_HZ(h: *mut Handle, out int nHZ) -> HRESULT;

    /* skip or bin */
    fn Toupcam_put_Mode(h: *mut Handle, int bSkip); -> int
    fn Toupcam_get_Mode(h: *mut Handle, out int bSkip) -> HRESULT;

    fn Toupcam_put_TempTint(h: *mut Handle, int nTemp, int nTint) -> HRESULT;
    fn Toupcam_get_TempTint(h: *mut Handle, out int nTemp, out int nTint) -> HRESULT;

    fn Toupcam_put_WhiteBalanceGain(h: *mut Handle, [In] int[] aGain) -> HRESULT;
    fn Toupcam_get_WhiteBalanceGain(h: *mut Handle, [Out] int[] aGain) -> HRESULT;

    fn Toupcam_put_AWBAuxRect(h: *mut Handle, ref RECT pAuxRect) -> HRESULT;
    fn Toupcam_get_AWBAuxRect(h: *mut Handle, out RECT pAuxRect) -> HRESULT;
    fn Toupcam_put_AEAuxRect(h: *mut Handle, ref RECT pAuxRect) -> HRESULT;
    fn Toupcam_get_AEAuxRect(h: *mut Handle, out RECT pAuxRect) -> HRESULT;

    /*
        S_FALSE:    color mode
        S_OK:       mono mode, such as EXCCD00300KMA and UHCCD01400KMA
    */
    fn Toupcam_get_MonoMode(h: *mut Handle) -> HRESULT;

    fn Toupcam_get_StillResolutionNumber(h: *mut Handle) -> uint;
    fn Toupcam_get_StillResolution(h: *mut Handle, uint nIndex, out int pWidth, out int pHeight) -> HRESULT;

    /*
        get the serial number which is always 32 chars which is zero-terminated such as "TP110826145730ABCD1234FEDC56787"
    */
    fn Toupcam_get_SerialNumber(h: *mut Handle, IntPtr sn) -> HRESULT;

    /*
        get the camera firmware version, such as: 3.2.1.20140922
    */
    fn Toupcam_get_FwVersion(h: *mut Handle, IntPtr fwver) -> HRESULT;
    /*
        get the camera hardware version, such as: 3.2.1.20140922
    */
    fn Toupcam_get_HwVersion(h: *mut Handle, IntPtr hwver) -> HRESULT;
    /*
        get the production date, such as: 20150327
    */
    fn Toupcam_get_ProductionDate(h: *mut Handle, IntPtr pdate) -> HRESULT;

    fn Toupcam_put_ExpoCallback(h: *mut Handle, PITOUPCAM_EXPOSURE_CALLBACK fnExpoProc, IntPtr pExpoCtx) -> HRESULT;
    fn Toupcam_put_ChromeCallback(h: *mut Handle, PITOUPCAM_CHROME_CALLBACK fnChromeProc, IntPtr pChromeCtx) -> HRESULT;
    fn Toupcam_AwbOnePush(h: *mut Handle, PITOUPCAM_TEMPTINT_CALLBACK fnTTProc, IntPtr pTTCtx) -> HRESULT;
    fn Toupcam_AwbInit(h: *mut Handle, PITOUPCAM_WHITEBALANCE_CALLBACK fnWBProc, IntPtr pWBCtx) -> HRESULT;
    fn Toupcam_LevelRangeAuto(h: *mut Handle) -> HRESULT;
    fn Toupcam_GetHistogram(h: *mut Handle, PITOUPCAM_HISTOGRAM_CALLBACK fnHistogramProc, IntPtr pHistogramCtx) -> HRESULT;

    fn Toupcam_put_LEDState(h: *mut Handle, ushort iLed, ushort iState, ushort iPeriod) -> HRESULT;

    fn Toupcam_write_EEPROM(h: *mut Handle, uint addr, IntPtr pData, uint nDataLen) -> HRESULT;
    fn Toupcam_read_EEPROM(h: *mut Handle, uint addr, IntPtr pBuffer, uint nBufferLen) -> HRESULT;

    fn Toupcam_put_Option(h: *mut Handle, eOPTION iOption, uint iValue) -> HRESULT;
    fn Toupcam_get_Option(h: *mut Handle, eOPTION iOption, out uint iValue) -> HRESULT;

    fn Toupcam_calc_ClarityFactor(IntPtr pImageData, int bits, uint nImgWidth, uint nImgHeight) -> double;
