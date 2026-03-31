<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Routing\Controller;

class RealtimeController extends Controller
{
    public function health()
    {
        return response()->json([
            'status' => 'ok',
            'service' => 'laravel-bridge'
        ]);
    }

    public function echo(Request $request)
    {
        return response()->json([
            'status' => 'ok',
            'payload' => $request->all()
        ]);
    }
}
