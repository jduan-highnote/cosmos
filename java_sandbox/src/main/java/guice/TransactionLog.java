package guice;

public interface TransactionLog {
    void logChargeResult(ChargeResult chargeResult);

    void logException(RuntimeException e);
}
