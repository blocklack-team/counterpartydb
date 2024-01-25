# This is an auto-generated Django model module.
# You'll have to do the following manually to clean this up:
#   * Rearrange models' order
#   * Make sure each model has one field with primary_key=True
#   * Make sure each ForeignKey and OneToOneField has `on_delete` set to the desired behavior
#   * Remove `managed = False` lines if you wish to allow Django to create, modify, and delete the table
# Feel free to rename the models, but don't rename db_table values or field names.
from django.db import models

""" 
first 22 bytes 24 in hex
434e54525052545900000014

prefix 8 bytes 16 in hex
434e545250525459
the ms id is in 9 byte, 17-18 in hex
00
so if the 9 byte is 0 ('00' in hex) just move 4 bytes to the right
14
int(0x14, 16) = 20
"""


class Addresses(models.Model):
    address = models.TextField(unique=True, blank=True, null=True)
    options = models.IntegerField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'addresses'


class Assets(models.Model):
    asset_id = models.TextField(unique=True, blank=True, null=True)
    asset_name = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    asset_longname = models.TextField(unique=True, blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'assets'


class Balances(models.Model):
    address = models.TextField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    quantity = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'balances'


class BetExpirations(models.Model):
    bet_index = models.AutoField(primary_key=True, blank=True, null=True)
    bet_hash = models.TextField(unique=True, blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    block_index = models.ForeignKey(
        'Blocks', models.DO_NOTHING, db_column='block_index', blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'bet_expirations'


class BetMatchExpirations(models.Model):
    bet_match = models.OneToOneField(
        'BetMatches', models.DO_NOTHING, primary_key=True, blank=True, null=True)
    tx0_address = models.TextField(blank=True, null=True)
    tx1_address = models.TextField(blank=True, null=True)
    block_index = models.ForeignKey(
        'Blocks', models.DO_NOTHING, db_column='block_index', blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'bet_match_expirations'


class BetMatchResolutions(models.Model):
    bet_match = models.OneToOneField(
        'BetMatches', models.DO_NOTHING, primary_key=True, blank=True, null=True)
    bet_match_type_id = models.IntegerField(blank=True, null=True)
    block_index = models.ForeignKey(
        'Blocks', models.DO_NOTHING, db_column='block_index', blank=True, null=True)
    winner = models.TextField(blank=True, null=True)
    settled = models.BooleanField(blank=True, null=True)
    bull_credit = models.IntegerField(blank=True, null=True)
    bear_credit = models.IntegerField(blank=True, null=True)
    escrow_less_fee = models.IntegerField(blank=True, null=True)
    fee = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'bet_match_resolutions'


class BetMatches(models.Model):
    id = models.TextField(primary_key=True, blank=True, null=True)
    tx0_index = models.IntegerField(blank=True, null=True)
    tx0_hash = models.TextField(blank=True, null=True)
    tx0_address = models.TextField(blank=True, null=True)
    tx1_index = models.IntegerField(blank=True, null=True)
    tx1_hash = models.TextField(blank=True, null=True)
    tx1_address = models.TextField(blank=True, null=True)
    tx0_bet_type = models.IntegerField(blank=True, null=True)
    tx1_bet_type = models.IntegerField(blank=True, null=True)
    feed_address = models.TextField(blank=True, null=True)
    initial_value = models.IntegerField(blank=True, null=True)
    deadline = models.IntegerField(blank=True, null=True)
    target_value = models.FloatField(blank=True, null=True)
    leverage = models.IntegerField(blank=True, null=True)
    forward_quantity = models.IntegerField(blank=True, null=True)
    backward_quantity = models.IntegerField(blank=True, null=True)
    tx0_block_index = models.IntegerField(blank=True, null=True)
    tx1_block_index = models.IntegerField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    tx0_expiration = models.IntegerField(blank=True, null=True)
    tx1_expiration = models.IntegerField(blank=True, null=True)
    match_expire_index = models.IntegerField(blank=True, null=True)
    fee_fraction_int = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'bet_matches'


class Bets(models.Model):
    tx_index = models.AutoField(unique=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    feed_address = models.TextField(blank=True, null=True)
    bet_type = models.IntegerField(blank=True, null=True)
    deadline = models.IntegerField(blank=True, null=True)
    wager_quantity = models.IntegerField(blank=True, null=True)
    wager_remaining = models.IntegerField(blank=True, null=True)
    counterwager_quantity = models.IntegerField(blank=True, null=True)
    counterwager_remaining = models.IntegerField(blank=True, null=True)
    target_value = models.FloatField(blank=True, null=True)
    leverage = models.IntegerField(blank=True, null=True)
    expiration = models.IntegerField(blank=True, null=True)
    expire_index = models.IntegerField(blank=True, null=True)
    fee_fraction_int = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'bets'


class Blocks(models.Model):
    block_index = models.AutoField(unique=True, blank=True, null=True)
    block_hash = models.TextField(unique=True, blank=True, null=True)
    block_time = models.IntegerField(blank=True, null=True)
    previous_block_hash = models.TextField(unique=True, blank=True, null=True)
    difficulty = models.IntegerField(blank=True, null=True)
    ledger_hash = models.TextField(blank=True, null=True)
    txlist_hash = models.TextField(blank=True, null=True)
    messages_hash = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'blocks'


class Broadcasts(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    timestamp = models.IntegerField(blank=True, null=True)
    value = models.FloatField(blank=True, null=True)
    fee_fraction_int = models.IntegerField(blank=True, null=True)
    text = models.TextField(blank=True, null=True)
    locked = models.BooleanField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'broadcasts'


class Btcpays(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    destination = models.TextField(blank=True, null=True)
    btc_amount = models.IntegerField(blank=True, null=True)
    order_match_id = models.TextField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'btcpays'


class Burns(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    burned = models.IntegerField(blank=True, null=True)
    earned = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'burns'


class Cancels(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    offer_hash = models.TextField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'cancels'


class Credits(models.Model):
    block_index = models.ForeignKey(
        Blocks, models.DO_NOTHING, db_column='block_index', blank=True, null=True)
    address = models.TextField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    quantity = models.IntegerField(blank=True, null=True)
    calling_function = models.TextField(blank=True, null=True)
    event = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'credits'


class Debits(models.Model):
    block_index = models.ForeignKey(
        Blocks, models.DO_NOTHING, db_column='block_index', blank=True, null=True)
    address = models.TextField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    quantity = models.IntegerField(blank=True, null=True)
    action = models.TextField(blank=True, null=True)
    event = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'debits'


class Destructions(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    asset = models.IntegerField(blank=True, null=True)
    quantity = models.IntegerField(blank=True, null=True)
    tag = models.TextField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'destructions'


class DispenserRefills(models.Model):
    tx_index = models.AutoField(blank=True, null=True)
    tx_hash = models.TextField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    destination = models.TextField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    dispense_quantity = models.IntegerField(blank=True, null=True)
    dispenser_tx_hash = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'dispenser_refills'


class Dispensers(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    give_quantity = models.IntegerField(blank=True, null=True)
    escrow_quantity = models.IntegerField(blank=True, null=True)
    satoshirate = models.IntegerField(blank=True, null=True)
    status = models.IntegerField(blank=True, null=True)
    give_remaining = models.IntegerField(blank=True, null=True)
    oracle_address = models.TextField(blank=True, null=True)
    last_status_tx_hash = models.TextField(blank=True, null=True)
    origin = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'dispensers'


class Dispenses(models.Model):
    tx_index = models.AutoField(blank=True, null=True)
    dispense_index = models.IntegerField(blank=True, null=True)
    tx_hash = models.TextField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    destination = models.TextField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    dispense_quantity = models.IntegerField(blank=True, null=True)
    dispenser_tx_hash = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'dispenses'


class Dividends(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    dividend_asset = models.TextField(blank=True, null=True)
    quantity_per_unit = models.IntegerField(blank=True, null=True)
    fee_paid = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'dividends'


class Issuances(models.Model):
    tx_index = models.AutoField(blank=True, null=True)
    tx_hash = models.TextField(blank=True, null=True)
    msg_index = models.IntegerField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    quantity = models.IntegerField(blank=True, null=True)
    divisible = models.BooleanField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    issuer = models.TextField(blank=True, null=True)
    transfer = models.BooleanField(blank=True, null=True)
    callable = models.BooleanField(blank=True, null=True)
    call_date = models.IntegerField(blank=True, null=True)
    call_price = models.FloatField(blank=True, null=True)
    description = models.TextField(blank=True, null=True)
    fee_paid = models.IntegerField(blank=True, null=True)
    locked = models.BooleanField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)
    asset_longname = models.TextField(blank=True, null=True)
    reset = models.BooleanField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'issuances'


class Mempool(models.Model):
    tx_hash = models.TextField(blank=True, null=True)
    command = models.TextField(blank=True, null=True)
    category = models.TextField(blank=True, null=True)
    bindings = models.TextField(blank=True, null=True)
    timestamp = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'mempool'


class Messages(models.Model):
    message_index = models.AutoField(primary_key=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    command = models.TextField(blank=True, null=True)
    category = models.TextField(blank=True, null=True)
    bindings = models.TextField(blank=True, null=True)
    timestamp = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'messages'


class OrderExpirations(models.Model):
    order_index = models.AutoField(primary_key=True, blank=True, null=True)
    order_hash = models.TextField(unique=True, blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    block_index = models.ForeignKey(
        Blocks, models.DO_NOTHING, db_column='block_index', blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'order_expirations'


class OrderMatchExpirations(models.Model):
    order_match = models.OneToOneField(
        'OrderMatches', models.DO_NOTHING, primary_key=True, blank=True, null=True)
    tx0_address = models.TextField(blank=True, null=True)
    tx1_address = models.TextField(blank=True, null=True)
    block_index = models.ForeignKey(
        Blocks, models.DO_NOTHING, db_column='block_index', blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'order_match_expirations'


class OrderMatches(models.Model):
    id = models.TextField(primary_key=True, blank=True, null=True)
    tx0_index = models.IntegerField(blank=True, null=True)
    tx0_hash = models.TextField(blank=True, null=True)
    tx0_address = models.TextField(blank=True, null=True)
    tx1_index = models.IntegerField(blank=True, null=True)
    tx1_hash = models.TextField(blank=True, null=True)
    tx1_address = models.TextField(blank=True, null=True)
    forward_asset = models.TextField(blank=True, null=True)
    forward_quantity = models.IntegerField(blank=True, null=True)
    backward_asset = models.TextField(blank=True, null=True)
    backward_quantity = models.IntegerField(blank=True, null=True)
    tx0_block_index = models.IntegerField(blank=True, null=True)
    tx1_block_index = models.IntegerField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    tx0_expiration = models.IntegerField(blank=True, null=True)
    tx1_expiration = models.IntegerField(blank=True, null=True)
    match_expire_index = models.IntegerField(blank=True, null=True)
    fee_paid = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'order_matches'


class Orders(models.Model):
    tx_index = models.AutoField(unique=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    give_asset = models.TextField(blank=True, null=True)
    give_quantity = models.IntegerField(blank=True, null=True)
    give_remaining = models.IntegerField(blank=True, null=True)
    get_asset = models.TextField(blank=True, null=True)
    get_quantity = models.IntegerField(blank=True, null=True)
    get_remaining = models.IntegerField(blank=True, null=True)
    expiration = models.IntegerField(blank=True, null=True)
    expire_index = models.IntegerField(blank=True, null=True)
    fee_required = models.IntegerField(blank=True, null=True)
    fee_required_remaining = models.IntegerField(blank=True, null=True)
    fee_provided = models.IntegerField(blank=True, null=True)
    fee_provided_remaining = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'orders'


class Rps(models.Model):
    tx_index = models.AutoField(unique=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    possible_moves = models.IntegerField(blank=True, null=True)
    wager = models.IntegerField(blank=True, null=True)
    move_random_hash = models.TextField(blank=True, null=True)
    expiration = models.IntegerField(blank=True, null=True)
    expire_index = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'rps'


class RpsExpirations(models.Model):
    rps_index = models.AutoField(primary_key=True, blank=True, null=True)
    rps_hash = models.TextField(unique=True, blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    block_index = models.ForeignKey(
        Blocks, models.DO_NOTHING, db_column='block_index', blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'rps_expirations'


class RpsMatchExpirations(models.Model):
    rps_match = models.OneToOneField(
        'RpsMatches', models.DO_NOTHING, primary_key=True, blank=True, null=True)
    tx0_address = models.TextField(blank=True, null=True)
    tx1_address = models.TextField(blank=True, null=True)
    block_index = models.ForeignKey(
        Blocks, models.DO_NOTHING, db_column='block_index', blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'rps_match_expirations'


class RpsMatches(models.Model):
    id = models.TextField(primary_key=True, blank=True, null=True)
    tx0_index = models.IntegerField(blank=True, null=True)
    tx0_hash = models.TextField(blank=True, null=True)
    tx0_address = models.TextField(blank=True, null=True)
    tx1_index = models.IntegerField(blank=True, null=True)
    tx1_hash = models.TextField(blank=True, null=True)
    tx1_address = models.TextField(blank=True, null=True)
    tx0_move_random_hash = models.TextField(blank=True, null=True)
    tx1_move_random_hash = models.TextField(blank=True, null=True)
    wager = models.IntegerField(blank=True, null=True)
    possible_moves = models.IntegerField(blank=True, null=True)
    tx0_block_index = models.IntegerField(blank=True, null=True)
    tx1_block_index = models.IntegerField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    tx0_expiration = models.IntegerField(blank=True, null=True)
    tx1_expiration = models.IntegerField(blank=True, null=True)
    match_expire_index = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'rps_matches'


class Rpsresolves(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    move = models.IntegerField(blank=True, null=True)
    random = models.TextField(blank=True, null=True)
    rps_match_id = models.TextField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'rpsresolves'


class Sends(models.Model):
    tx_index = models.AutoField(blank=True, null=True)
    tx_hash = models.TextField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    destination = models.TextField(blank=True, null=True)
    asset = models.TextField(blank=True, null=True)
    quantity = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)
    msg_index = models.IntegerField(blank=True, null=True)
    memo = models.BinaryField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'sends'


class Sweeps(models.Model):
    tx_index = models.AutoField(primary_key=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    destination = models.TextField(blank=True, null=True)
    flags = models.IntegerField(blank=True, null=True)
    status = models.TextField(blank=True, null=True)
    memo = models.BinaryField(blank=True, null=True)
    fee_paid = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'sweeps'


class TransactionOutputs(models.Model):
    # This field type is a guess.
    tx_index = models.TextField(blank=True, null=True)
    tx_hash = models.TextField(blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    out_index = models.IntegerField(blank=True, null=True)
    destination = models.TextField(blank=True, null=True)
    btc_amount = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'transaction_outputs'


class Transactions(models.Model):
    tx_index = models.AutoField(unique=True, blank=True, null=True)
    tx_hash = models.TextField(unique=True, blank=True, null=True)
    block_index = models.IntegerField(blank=True, null=True)
    block_hash = models.TextField(blank=True, null=True)
    block_time = models.IntegerField(blank=True, null=True)
    source = models.TextField(blank=True, null=True)
    destination = models.TextField(blank=True, null=True)
    btc_amount = models.IntegerField(blank=True, null=True)
    fee = models.IntegerField(blank=True, null=True)
    data = models.BinaryField(blank=True, null=True)
    supported = models.BooleanField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'transactions'


class Undolog(models.Model):
    undo_index = models.AutoField(primary_key=True, blank=True, null=True)
    sql = models.TextField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'undolog'


class UndologBlock(models.Model):
    block_index = models.AutoField(primary_key=True, blank=True, null=True)
    first_undo_index = models.IntegerField(blank=True, null=True)

    class Meta:
        managed = False
        db_table = 'undolog_block'
