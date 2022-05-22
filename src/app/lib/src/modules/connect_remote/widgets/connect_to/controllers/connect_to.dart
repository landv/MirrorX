import 'dart:developer';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:app/src/controllers/mirrorx_core.dart';
import 'package:app/src/controllers/page_view.dart';
import 'package:app/src/core/utils/dialog.dart';
import 'package:app/src/modules/connect_remote/widgets/connect_to/controllers/chars_input.dart';

class ConnectToController extends GetxController {
  late CharacterInputController _digitInputController;
  late MirrorXCoreController _sdk;
  late PageViewController _pageViewController;

  bool _isLoading = false;

  bool get isLoading => _isLoading;

  @override
  void onInit() {
    _digitInputController = Get.put(CharacterInputController());
    _sdk = Get.find();
    _pageViewController = Get.find();
    super.onInit();
  }

  Future<void> connectTo() async {
    final deviceID = _digitInputController.inputText;
    if (deviceID == null || deviceID.isEmpty) {
      popupErrorDialog(content: "connect_to_remote.dialog.empty_input".tr);
      return;
    }

    if (deviceID.length != 8) {
      popupErrorDialog(content: "connect_to_remote.dialog.invalid_length".tr);
      return;
    }

    if (!RegExp(r'^[1-9a-hjkmnp-zA-HJKMNP-Z]+$').hasMatch(deviceID)) {
      popupErrorDialog(content: "connect_to_remote.dialog.invalid_char".tr);
      return;
    }

    final deviceRunesList = deviceID.runes.toList();

    for (var ch in deviceRunesList) {
      if (deviceRunesList.indexOf(ch) != deviceRunesList.lastIndexOf(ch)) {
        popupErrorDialog(
            content: "connect_to_remote.dialog.invalid_format.repeat_char".tr);
        return;
      }
    }

    try {
      _isLoading = true;
      update();

      await _sdk.getInstance().desktopConnect(remoteDeviceId: deviceID);
      _popupDesktopConnectInputPasswordDialog(deviceID);
    } catch (err) {
      log("desktop connect failed", error: err);
      popupErrorDialog(content: "connect_to_remote.dialog.disallow".tr);
    } finally {
      _isLoading = false;
      update();
    }
  }

  Future<void> authPassword(
      TextEditingController controller, String deviceID) async {
    if (controller.text.isEmpty) {
      return;
    }

    try {
      final passwordCorrect = await _sdk
          .getInstance()
          .desktopKeyExchangeAndPasswordVerify(
              remoteDeviceId: deviceID, password: controller.text);

      if (!passwordCorrect) {
        popupErrorDialog(
            content: "connect_to_remote.dialog.incorrect_password".tr);
        return;
      }

      final reply = await _sdk
          .getInstance()
          .desktopStartMediaTransmission(remoteDeviceId: deviceID);

      log("start media transmission: os_name=${reply.osName}, os_version=${reply.osVersion}, video_=${reply.videoType}, audio_=${reply.audioType}");

      _pageViewController.addRemoteDesktopPage(
          deviceID, reply.osName, reply.osVersion);
    } catch (err) {
      log("desktop connect failed", error: err);
      popupErrorDialog(content: "connect_to_remote.dialog.another_error".tr);
    }
  }

  void _popupDesktopConnectInputPasswordDialog(String deviceID) {
    final passwordTextController = TextEditingController();

    Get.defaultDialog(
        title: "MirrorX",
        titleStyle: const TextStyle(fontSize: 18),
        content: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Padding(
              padding: const EdgeInsets.only(bottom: 8.0),
              child: Text(
                "请输入设备[$deviceID]的访问密码",
                textAlign: TextAlign.center,
                style: const TextStyle(fontSize: 16),
              ),
            ),
            CupertinoTextField(
              controller: passwordTextController,
              textAlign: TextAlign.center,
              maxLength: 16,
              maxLines: 1,
            ),
          ],
        ),
        contentPadding: const EdgeInsets.fromLTRB(16, 8, 16, 8),
        barrierDismissible: false,
        radius: 12,
        actions: [
          TextButton(
              onPressed: () async {
                Get.back(closeOverlays: true);
                await authPassword(passwordTextController, deviceID);
              },
              child: Text("dialog.ok".tr)),
          TextButton(
              onPressed: () {
                Get.back(closeOverlays: true);
              },
              child: Text("dialog.cancel".tr))
        ]);
  }
}