/*-
 * Free/Libre Near Field Communication (NFC) library
 *
 * Libnfc historical contributors:
 * Copyright (C) 2009      Roel Verdult
 * Copyright (C) 2009-2013 Romuald Conty
 * Copyright (C) 2010-2012 Romain Tartière
 * Copyright (C) 2010-2013 Philippe Teuwen
 * Copyright (C) 2012-2013 Ludovic Rousseau
 * See AUTHORS file for a more comprehensive list of contributors.
 * Additional contributors of this file:
 * Copyright (C) 2020      Adam Laurie
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *  1) Redistributions of source code must retain the above copyright notice,
 *  this list of conditions and the following disclaimer.
 *  2 )Redistributions in binary form must reproduce the above copyright
 *  notice, this list of conditions and the following disclaimer in the
 *  documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * Note that this license only applies on the examples, NFC library itself is
 * under LGPL
 *
 */

/**
 * @file nfc-poll.c
 * @brief Polling example
 */

#ifdef HAVE_CONFIG_H
#include "config.h"
#endif // HAVE_CONFIG_H

#include <err.h>
#include <inttypes.h>
#include <signal.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include <cjson/cJSON.h>
#include <nfc/nfc-types.h>
#include <nfc/nfc.h>

// #include "utils/nfc-utils.h"

#define MAX_DEVICE_COUNT 16
#define ERR(x, y) printf(x, y);

static nfc_device *pnd = NULL;
static nfc_context *context;

static void stop_polling(int sig) {
  (void)sig;
  if (pnd != NULL)
    nfc_abort_command(pnd);
  else {
    nfc_exit(context);
    exit(EXIT_FAILURE);
  }
}

const char *nfc_poll_tag() {
  signal(SIGINT, stop_polling);

  const uint8_t uiPollNr = 20;
  const uint8_t uiPeriod = 2;
  const nfc_modulation nmModulations[3] = {
      {.nmt = NMT_ISO14443A, .nbr = NBR_106},
      // {.nmt = NMT_ISO14443B, .nbr = NBR_106},
      {.nmt = NMT_FELICA, .nbr = NBR_212},
      {.nmt = NMT_FELICA, .nbr = NBR_424},
      // {.nmt = NMT_JEWEL, .nbr = NBR_106},
      // {.nmt = NMT_ISO14443BICLASS, .nbr = NBR_106},
  };
  const size_t szModulations = 3;

  nfc_target nt;
  int res = 0;

  nfc_init(&context);
  if (context == NULL) {
    ERR("%s", "Unable to init libnfc (malloc)");
    exit(EXIT_FAILURE);
  }

  pnd = nfc_open(context, NULL);

  if (pnd == NULL) {
    ERR("%s", "Unable to open NFC device.");
    nfc_exit(context);
    exit(EXIT_FAILURE);
  }

  if (nfc_initiator_init(pnd) < 0) {
    nfc_perror(pnd, "nfc_initiator_init");
    nfc_close(pnd);
    nfc_exit(context);
    exit(EXIT_FAILURE);
  }

  printf("NFC reader: %s opened\n", nfc_device_get_name(pnd));
  printf(
      "NFC device will poll during %ld ms (%u pollings of %lu ms for %" PRIdPTR
      " modulations)\n",
      (unsigned long)uiPollNr * szModulations * uiPeriod * 150, uiPollNr,
      (unsigned long)uiPeriod * 150, szModulations);
  if ((res = nfc_initiator_poll_target(pnd, nmModulations, szModulations,
                                       uiPollNr, uiPeriod, &nt)) < 0) {
    nfc_perror(pnd, "nfc_initiator_poll_target");
    nfc_close(pnd);
    nfc_exit(context);
    exit(EXIT_FAILURE);
  }

  cJSON *json_tag = cJSON_CreateObject();
  cJSON *type = NULL;
  // cJSON *ats = NULL;

  if (nt.nm.nmt == NMT_ISO14443A) {
    printf("Found ISO14443A card");
    type = cJSON_CreateString("ISO14443A");
    // Fill ATQA
    cJSON *atqa = cJSON_CreateArray();
    cJSON *atqa_1 = cJSON_CreateNumber(nt.nti.nai.abtAtqa[0]);
    cJSON *atqa_2 = cJSON_CreateNumber(nt.nti.nai.abtAtqa[1]);
    cJSON_AddItemToArray(atqa, atqa_1);
    cJSON_AddItemToArray(atqa, atqa_2);

    // Fill SAK
    cJSON *sak = cJSON_CreateNumber(nt.nti.nai.btSak);

    // Fill UID
    cJSON *uid = cJSON_CreateArray();
    for (size_t i=0; i<nt.nti.nai.szUidLen; i++) {
      cJSON *el = cJSON_CreateNumber(nt.nti.nai.abtUid[i]);
      cJSON_AddItemToArray(uid, el);
    }

    // Finish
    cJSON_AddItemToObject(json_tag, "atqa", atqa);
    cJSON_AddItemToObject(json_tag, "sak", sak);
    cJSON_AddItemToObject(json_tag, "uid", uid);
  } else if(nt.nm.nmt == NMT_FELICA) {
    type = cJSON_CreateString("FeliCa");
    cJSON *uid = cJSON_CreateArray();
    for (size_t i= 0; i<8; i++) {
      cJSON *uid_num = cJSON_CreateNumber(nt.nti.nfi.abtId[i]);
      cJSON_AddItemToArray(uid, uid_num);
    }
    cJSON *pad = cJSON_CreateArray();
    for (size_t i= 0; i<8; i++) {
      cJSON *pad_num = cJSON_CreateNumber(nt.nti.nfi.abtPad[i]);
      cJSON_AddItemToArray(pad, pad_num);
    }
    cJSON *sys_code = cJSON_CreateArray();
    for (size_t i= 0; i<8; i++) {
      cJSON *sys_code_num = cJSON_CreateNumber(nt.nti.nfi.abtSysCode[i]);
      cJSON_AddItemToArray(sys_code, sys_code_num);
    }
    cJSON_AddItemToObject(json_tag, "uid", uid);
    cJSON_AddItemToObject(json_tag, "pad", pad);
    cJSON_AddItemToObject(json_tag, "sys_code", sys_code);
  }

  cJSON_AddItemToObject(json_tag, "type", type);

  if (res > 0) {
    // Found
    nfc_perror(pnd, "nfc_initiator_target_is_present");
    printf("done.\n");
  } else {
    printf("No target found.\n");
  }

  nfc_close(pnd);
  nfc_exit(context);

  const char *result = cJSON_Print(json_tag);
  printf("%s\n", result);
  return result;
  // exit(EXIT_SUCCESS);
}