<?php

use Illuminate\Support\Facades\Route;
use App\Http\Controllers\RealtimeController;

Route::get('/bridge/health', [RealtimeController::class, 'health']);
Route::post('/bridge/echo', [RealtimeController::class, 'echo']);
