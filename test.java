    @POST("user/card/add")
    Observable<List<CreditCard>> addCard(@Body StripeToken stripeToken);

    @POST("pay/methods/add")
    Observable<JsonObject> addCardWithIntent(@Body StripeToken stripeToken);

    @POST("trips/add_points")
    Observable<Response<JsonObject>> addPointsInRide(@Body CurrentTripId currentTripId);

    @POST("scooters/sync")
    Observable<CurrentRide> addPointsWithSync(@Body CurrentTripId currentTripId);

    @POST("user/promo")
    Observable<UserQuery> addPromoCode(@Body ReferralCode referralCode);

    @POST("user/terms")
    Observable<Response<JsonObject>> agreeOnTerms(@Body JsonObject jsonObject);

    @POST("pay/package")
    Observable<JsonObject> buyPackage(@Body PackageBody packageBody);

    @POST("pay/tinaba?aliPay=true")
    Observable<AliModel> buyPackageWithAli(@Body PackageBody packageBody);

    @POST("pay/tinaba")
    Observable<TinabaModel> buyPackageWithTinaba(@Body PackageBody packageBody);

    @GET("user/tokens/buy_form")
    Observable<Response<JsonObject>> buyToken(@Query("amount") String str, @Query("language") String str2);

    @POST("user/subscriptions/cancel")
    Observable<JsonObject> cancelSubscription(@Body CancelReason cancelReason);

    @POST("vehicles/speed")
    Observable<AccelerationModeResponse> changeAccelerationMode(@Body AccelerationMode accelerationMode);

    @GET("user/check/{email}")
    Observable<Response<JsonObject>> checkIfUserExists(@Path("email") String str);

    @GET("user/terms")
    Observable<Response<RegionTerms>> checkTerms(@Query("location") String str, @Query("language") String str2);

    @GET("config")
    Observable<Config> config();

    @POST("user/create")
    Observable<Response<UserCreatedResponse>> createUser(@Body User user);

    @POST("user/delete")
    Observable<Response<UserQuery>> deleteUser();

    @GET("user/facebook/auth/{token}")
    Observable<Response<JsonObject>> getAuthTokenFacebook(@Path("token") String str);

    @GET("regions")
    Observable<List<Region>> getAvailableRegions();

    @GET("regions")
    Observable<List<Region>> getAvailableRegions(@Query("location") String str);

    @GET(Station.VEHICLES)
    Observable<List<Vehicle>> getAvailableScooters(@Query("northWest") String str, @Query("southEast") String str2);

    @GET("geo/tiles")
    Observable<GeoJsonResponse> getGeoJson(@Query("location") String str);

    @GET("screens")
    Observable<List<InfoScreen>> getInfoScreens(@Query("location") String str);

